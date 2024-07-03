# What it is
This small programm count lines of text in utf-8 format. The input takes the path to the file or folder, as well as the file extension. The output will be a hashtable, where the key is the file extension, and its value is the number of lines for all files with this extension. The directory is traversed recursively.
# Example
`./count_lines . rs` \
`{"rs": 88}` \
`./count_lines ../frontend ts css js html` \
`{"ts": 565601, "css": 11356, "js": 4619028, "html": 19100}`