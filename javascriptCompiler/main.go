package main

import (
    "fmt"
    "log"
    "os"
	"regexp"
	"container/list"
)

type Position struct {
	row int
	column int
}

type Token struct {
	position Position
	name string
	value string
}


func cat(path string) {
    content, err := os.ReadFile(path)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(string(content))

}

func getFileContent(path string) string {
	content, err := os.ReadFile(path)
    if err != nil {
        log.Fatal(err)
		return "wrong"
	}
	return string(content)
}

func parse(content string) *list.List {
	tokens := list.New()

	var number = regexp.MustCompile(`^[-+]?\d+$`)
	var string_pattern = regexp.MustCompile(`\"[a-zA-Z]*"`)

	fmt.Println("Parsing file content")
	counter := 0
	row := 0
	column := 0

	for i, c := range content {
		token:= ""
		if c== '\n' || c == ';' || c == '\r' || c == '\t' {
			row++;
			column = 0;
			continue;
		}

		if c == '=' {
			token = string(c);
			v:= Token {
				position: Position { row, column },
				name: "assign",
				value: token,
			}
			tokens.PushBack(v)
			continue;
		}

		if c == '>' || c == '<' {
			token = string(c)
			v:= Token {
				position: Position { row, column },
				name: "compare",
				value: token,
			}
			tokens.PushBack(v)
			continue;
		}

		if c == '(' || c == ')' || c == '{' || c == '}' {
			token = string(c);

			v:= Token {
				position: Position { row, column },
				name: "par",
				value: token,
			}
			// fmt.Println(v)
			tokens.PushBack(v)
		} else if c == ' ' || c == '(' || c == ';' {
			token = content[counter:i];
			if number.MatchString(token) {
				v:= Token {
					position: Position { row, column },
					name: "number",
					value: token,
				}
				tokens.PushBack(v)
				// fmt.Println(v)
			} else if string_pattern.MatchString(token) {
				v:= Token {
					position: Position { row, column },
					name: "stringLiteral",
					value: token,
				}
				// fmt.Println(v)
				tokens.PushBack(v)
			} else {
				v:= Token {
					position: Position { row, column },
					name: "keyword",
					value: token,
				}
				// fmt.Println(v)
				tokens.PushBack(v)
			}
			counter = i;
		} 
		column++;
	}

	return tokens
}	

func main() {
	fmt.Println("### JavaScript Compiler to perl InBytecode ###")
	
	if len(os.Args) == 1 {
		fmt.Println("Usage ./main <entrypoint>")
		return
	}

	argsWithProg := os.Args
    pathname := argsWithProg[1]

    fileContent := getFileContent(pathname)
	tokens := parse(fileContent)

	for e := tokens.Front(); e != nil; e = e.Next() {
		fmt.Println(e.Value)
	}
}
