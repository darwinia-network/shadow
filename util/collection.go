package util

// Filter string array
func Filter(vs []string, f func(int, string) bool) []string {
	vsf := make([]string, 0)
	for i, v := range vs {
		if f(i, v) {
			vsf = append(vsf, v)
		}
	}
	return vsf
}

// Map string array
func Map(vs []string, f func(int, string) string) []string {
	vsm := make([]string, len(vs))
	for i, v := range vs {
		vsm[i] = f(i, v)
	}
	return vsm
}
