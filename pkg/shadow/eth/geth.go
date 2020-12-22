package eth

import (
    "encoding/binary"
    "fmt"
    "os"
    "path/filepath"
    "github.com/darwinia-network/shadow/pkg/log"
    "github.com/golang/snappy"
    "github.com/syndtr/goleveldb/leveldb"
)

const indexEntrySize = 6

var (
    headerPrefix       = []byte("h")
    headerHashSuffix   = []byte("n")
    headHeaderKey = []byte("LastHeader")
    headerNumberPrefix = []byte("H")
)

type indexEntry struct {
    filenum uint32 // stored as uint16 ( 2 bytes)
    offset  uint32 // stored as uint32 ( 4 bytes)
}

func (i *indexEntry) unmarshalBinary(b []byte) {
    i.filenum = uint32(binary.BigEndian.Uint16(b[:2]))
    i.offset = binary.BigEndian.Uint32(b[2:6])
}

func headerHashKey(number uint64) []byte {
    return append(append(headerPrefix, encodeBlockNumber(number)...), headerHashSuffix...)
}

func headerNumberKey(hash []byte) []byte {
    return append(headerNumberPrefix, hash...)
}

func encodeBlockNumber(number uint64) []byte {
    enc := make([]byte, 8)
    binary.BigEndian.PutUint64(enc, number)
    return enc
}

func getBounds(indexfile *os.File, item uint64) (uint32, uint32, uint32, error) {
    buffer := make([]byte, indexEntrySize)
    var startIdx, endIdx indexEntry
    if _, err := indexfile.ReadAt(buffer, int64((item+1)*indexEntrySize)); err != nil {
        return 0, 0, 0, err
    }
    endIdx.unmarshalBinary(buffer)
    if item != 0 {
        if _, err := indexfile.ReadAt(buffer, int64(item*indexEntrySize)); err != nil {
            return 0, 0, 0, err
        }
        startIdx.unmarshalBinary(buffer)
    } else {
        return 0, endIdx.offset, endIdx.filenum, nil
    }
    if startIdx.filenum != endIdx.filenum {
        return 0, endIdx.offset, endIdx.filenum, nil
    }
    return startIdx.offset, endIdx.offset, endIdx.filenum, nil
}

type AncientReader struct {
    files map[uint32]*os.File
    index *os.File
    compression bool
    dir string
    name string
}

func NewAncientReader(path string, name string, compression bool) *AncientReader {
    var idxName string
    if compression {
        idxName = filepath.Join(path, fmt.Sprintf("%s.cidx", name))
    } else {
        idxName = filepath.Join(path, fmt.Sprintf("%s.ridx", name))
    }
    index, err := os.OpenFile(idxName, os.O_RDONLY, 0644)
    if err != nil {
        log.Error("open index file failed file %v, err %v", idxName, err)
        return nil
    }

    return &AncientReader {
        files: make(map[uint32]*os.File),
        index: index,
        compression: compression,
        dir: path,
        name: name,
    }
}

func (ar *AncientReader) getFilePtr(filenum uint32) (*os.File, error) {
    file, ok := ar.files[filenum]
    if ok {
        return file, nil
    }
    var fileName string
    if ar.compression {
        fileName = filepath.Join(ar.dir, fmt.Sprintf("%s.%04d.cdat", ar.name, filenum))
    } else {
        fileName = filepath.Join(ar.dir, fmt.Sprintf("%s.%04d.rdat", ar.name, filenum))
    }
    dataFile, err := os.OpenFile(fileName, os.O_RDONLY, 0644)
    if err != nil {
        return nil, err
    }
    ar.files[filenum] = dataFile
    return dataFile, nil
}

func (ar *AncientReader) Read(item uint64) (blob []byte, err error) {
    startOffset, endOffset, filenum, err := getBounds(ar.index, item)
    if err != nil {
        return nil, err
    }
    file, err := ar.getFilePtr(filenum)
    if err != nil {
        return nil, err
    }
    blob = make([]byte, endOffset-startOffset)
    if _, err := file.ReadAt(blob, int64(startOffset)); err != nil {
        return nil, err
    }
    if ar.compression {
        return snappy.Decode(nil, blob)
    }
    return blob, nil
}

type BlockHashReader struct {
    ar     *AncientReader
    db     *leveldb.DB
    arMax  uint64
    Head   uint64
}

func NewBlockHashReader(ar *AncientReader, dbpath string) *BlockHashReader {
    db, err := leveldb.OpenFile(dbpath, nil)
    if err != nil {
        log.Error("open geth db failed, path %v, err %v", dbpath, err)
        return nil
    }
    headHeader, err := db.Get(headHeaderKey, nil)
    if err != nil {
        log.Error("get headHeaderKey failed err %v", err)
        return nil
    }
    headNumber, err := db.Get(headerNumberKey(headHeader), nil)
    if err != nil {
        log.Error("get headHeaderNumber failed err %v", err)
        return nil
    }

    return &BlockHashReader {
        ar: ar,
        db: db,
        arMax: 0,
        Head: binary.BigEndian.Uint64(headNumber),
    }
}

func (bhr *BlockHashReader) Read(number uint64) (blob []byte, err error) {
    if bhr.arMax == 0 || number < bhr.arMax {
        blob, err = bhr.ar.Read(number)
        if err == nil {
            return
        } else {
            bhr.arMax = number
            log.Info("read geth max ar number %v, head number %v", number, bhr.Head)
        }
    }
    hashKey := headerHashKey(number)
    return bhr.db.Get(hashKey, nil)
}

