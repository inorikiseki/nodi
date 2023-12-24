#include <string>
#include <vector>
#include <map>

typedef struct {
  std::vector<Meta*> relatives;
} Meta;

template<View>
inline View* AsPtr(Meta* focused){
  return (View*)focusesd; 
}

const char* source= R"
Meta
  Diffum: 0 1
  Detum: determined undetermined
  bool: [0 1]
  bit: [0 1]
"; 

typedef struct {
  const char* source;
  Meta* focused;
  int line_distance;
} Parser;

Parser* newParser(const char* source) {
  Parser parser;
  parser.source = sourse;
  parser.focused = AsPtr<Meta>(source); 
  parser.line_distance = 0; 
  return &parser;
}

Parser* parser = newParser(const char* source);
:
inline bool is_whitespace(char c) {
    if(c == ' ' || c == '\t' || c == '\n' || c == '\r'){
        return true;
    } else {
        return false;
    }
}

inline bool eof(char c) {
  return c == '\0'; 
}

Meta* NextNode(Parser* parser) {
  int offset = 0;
  for (int i = 0; parser->focused[i] != '\0'; ++i) {
    match parser->focused[i]
  }

}
