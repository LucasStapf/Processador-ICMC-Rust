/// Label.
pub struct Label {
    pub name: String,
    pub addr: u16,
}

impl Label {
    pub fn new(name: String, addr: u16) -> Self {
        Self { name, addr }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
