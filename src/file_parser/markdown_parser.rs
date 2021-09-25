pub struct MarkdownDocument<'a> {
    // the bool refers whether the element is open
    elements: Vec<(BlockElement<'a>, bool)>
}

impl <'a> MarkdownDocument<'a> {
    pub fn new() -> MarkdownDocument<'a> {
        MarkdownDocument {
            elements: vec![]
        }
    }
    
    pub fn add_line_to_document(&mut self, line: &'a str) {
        let new_element = BlockElement::from(line);
        
        if self.elements.len() == 0 && new_element.is_some() {
            let new_element = new_element.unwrap();
            if new_element.is_heading() {
                self.elements.push((new_element, false));
            } else {
                self.elements.push((new_element, true));
            }
        } else if self.elements.len() != 0 && new_element.is_some() {
            let new_element = new_element.unwrap();
            let mut opened_element: Option<&mut (BlockElement<'a>, bool)> = None;
            
            for element in self.elements.iter_mut() {
                if element.1 {  // if element is open
                    opened_element = Some(element);
                }
            }
            
            if opened_element.is_some() {
                let mut opened_element = opened_element.unwrap();
                if new_element.is_heading() {
                    opened_element.1 = false;
                    self.elements.push((new_element, false));
                } else {
                    opened_element.0.merge(new_element);
                }
            } else {
                if new_element.is_heading() {
                    self.elements.push((new_element, false));
                } else {
                    self.elements.push((new_element, true));
                }
            }
        } else if self.elements.len() != 0 && new_element.is_none() {
            for element in self.elements.iter_mut() {
                if element.1 {  // if element is open
                    element.1 = false;
                }
            }
        }
    }
    pub fn print(&self) -> String {
        let mut result = String::new();
        for element in self.elements.iter() {
            result += element.0.print().as_str();
        }
        result
    }
}

enum BlockElement<'a> {
    Heading(usize, Vec<InlineElement<'a>>),
    Paragraph(Vec<InlineElement<'a>>)
}

impl <'a> BlockElement<'a> {
    pub fn from(line: &'a str) -> Option<BlockElement<'a>> {
        let heading = BlockElement::to_heading(line);
    
        if heading.is_some() {
            return heading;
        }
        
        BlockElement::to_paragraph(line)
    }
    
    fn to_heading(line: &'a str) -> Option<BlockElement<'a>> {
        let relevant_line_portion = trim_start_at_most(line, ' ', 3);
        if !relevant_line_portion.starts_with('#') {
            return None;
        }
        
        let text_after_hashtag_run = trim_start_at_most(relevant_line_portion, '#', 6);
        
        if !text_after_hashtag_run.is_empty() &&
            !text_after_hashtag_run.starts_with(| c: char | c.is_whitespace()) {
            return None;
        }
        
        let heading_level = relevant_line_portion.len() - text_after_hashtag_run.len();
        
        let trailing_whitespace_trimmed_heading = text_after_hashtag_run
            .trim_start()
            .trim_end();
        
        let trailing_hashtag_trimmed_heading = trailing_whitespace_trimmed_heading.trim_end_matches('#');
        
        if !trailing_hashtag_trimmed_heading.is_empty() &&
            !text_after_hashtag_run.starts_with(| c: char | c.is_whitespace()) {
                return Some(BlockElement::Heading(
                    heading_level,
                    vec![InlineElement::Text(trailing_whitespace_trimmed_heading)]));
        }
        
        Some(BlockElement::Heading(
            heading_level,
            vec![InlineElement::Text(trailing_hashtag_trimmed_heading.trim_end())]
        ))
    }
    
    fn to_paragraph(line: &'a str) -> Option<BlockElement<'a>> {
        if line.trim().is_empty() {
            None
        } else {
            Some(BlockElement::Paragraph(vec![InlineElement::Text(line.trim())]))
        }
    }
    
    pub fn merge(&mut self, element: BlockElement<'a>) {
        if let BlockElement::Paragraph(original_elements) = self {
            if let BlockElement::Paragraph(mut new_elements) = element {
                original_elements.push(InlineElement::SoftBreak);
                original_elements.append(&mut new_elements);
            }
        }
    }
    
    pub fn is_heading(&self) -> bool {
        matches!(*self, BlockElement::Heading(_, _))
    }
    
    pub fn print(&self) -> String {
        match self {
            BlockElement::Heading(heading_level, heading_text) => {
                format!("<h{level}>{inner}</h{level}>",
                    level = heading_level,
                    inner = heading_text.iter()
                        .map(|e| e.print())
                        .fold(String::new(),
                        |acc, s| acc + &s))
            },
            BlockElement::Paragraph(paragraph_text) => {
                format!("<p>{}</p>",
                    paragraph_text.iter()
                        .map(|e| e.print())
                        .fold(String::new(),
                        |acc, s| acc + &s))
            }
        }
    }
}

enum InlineElement<'a> {
    Text(&'a str),
    SoftBreak
}

impl <'a> InlineElement<'a> {
    fn print(&self) -> String {
        match *self {
            InlineElement::Text(text) => format!("{}", text),
            InlineElement::SoftBreak => format!(" ")
        }
    }
}

fn trim_start_at_most(line: &str, character_to_skip: char, number_of_times: usize) -> &str {
    let mut start_index = 0;
    
    for (i, c) in line.char_indices() {
        if start_index == number_of_times || c != character_to_skip {
            break;
        }
        
        start_index += 1;
    }
    
    &line[start_index..line.len()]
}

fn trim_end_at_most(line: &str, character_to_skip: char, number_of_times: usize) -> &str {
    let mut offset_from_end = 0;
    
    for (i, c) in line.char_indices().rev() {
        if offset_from_end == number_of_times || c != character_to_skip {
            break;
        }
        
        offset_from_end += 1;
    }
    
    &line[0..(line.len() - offset_from_end)]
}
