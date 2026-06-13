#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MathStyle {
    Display,
    Text,
    Script,
    ScriptScript,
}

impl MathStyle {
    pub fn font_size_factor(&self) -> f64 {
        match self {
            MathStyle::Display      => 1.0,
            MathStyle::Text         => 1.0,
            MathStyle::Script       => 0.7,
            MathStyle::ScriptScript => 0.5,
        }
    }

    pub fn cramped(&self) -> MathStyle {
        match self {
            MathStyle::Display      => MathStyle::Text,
            MathStyle::Text         => MathStyle::Text,
            MathStyle::Script       => MathStyle::Script,
            MathStyle::ScriptScript => MathStyle::ScriptScript,
        }
    }

    pub fn superscript_style(&self) -> MathStyle {
        match self {
            MathStyle::Display | MathStyle::Text => MathStyle::Script,
            MathStyle::Script | MathStyle::ScriptScript => MathStyle::ScriptScript,
        }
    }

    pub fn subscript_style(&self) -> MathStyle {
        self.superscript_style().cramped()
    }

    pub fn numerator_style(&self) -> MathStyle {
        match self {
            MathStyle::Display => MathStyle::Text,
            MathStyle::Text    => MathStyle::Script,
            _                  => MathStyle::ScriptScript,
        }
    }

    pub fn denominator_style(&self) -> MathStyle {
        self.numerator_style().cramped()
    }

    pub fn is_display(&self) -> bool {
        matches!(self, MathStyle::Display)
    }
}
