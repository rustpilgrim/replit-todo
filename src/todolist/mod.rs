pub struct TodoItem {
    pub text: String,
    pub status: bool,
}

pub struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoItem {
    pub fn new(text: &str) -> TodoItem {
        TodoItem {
            text: text.to_string(),
            status: false,
        }
    }

    pub fn check_status(&self) -> bool {
        self.status
    }

    pub fn toggle_status(&mut self) {
        self.status = !self.status;
    }
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    pub fn add_item(mut self, item: TodoItem) -> Result<String, String> {
        if item.text.is_empty() {
            return Err("Item text cannot be empty!".to_string());
        }
        self.items.push(item);
        Ok("".to_string())
    }

    pub fn remove_item(mut self, index: usize) -> Result<String, String> {
        if index >= self.items.len() {
            return Err("Index out of bounds!".to_string());
        }
        let item = self.items.remove(index);
        Ok(format!("Removed item: {}", item.text))
    }
}
