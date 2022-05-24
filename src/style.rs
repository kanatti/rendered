use std::collections::HashMap;

use crate::{
    css::{Rule, Selector, SimpleSelector, Specificity, StyleSheet, Value},
    dom::{ElementData, Node, NodeType},
};

pub type PropertyMap = HashMap<String, Value>;

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub css_props: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

pub enum Display {
    Block,
    Inline,
    None,
}

impl<'a> StyledNode<'a> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.css_props.get(name).map(|v| v.clone())
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }
}

// We ned to track specificity of matched selector for sorting rules before applying.
pub type MatchedRule<'a> = (Specificity, &'a Rule);

/// If given DOM matches a selector or not
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

// Matches for SimpleSelector variant
fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    if let Some(ref tag_name) = selector.tag_name {
        if *tag_name != *elem.tag_name() {
            return false;
        }
    }

    if let (&Some(ref selector_id), Some(elem_id)) = (&selector.id, elem.id()) {
        if *selector_id != *elem_id {
            return false;
        }
    }

    let elem_classes = elem.classes();
    if selector
        .classes
        .iter()
        .any(|class| elem_classes.contains(&class.as_str()))
    {
        return false;
    }

    return true;
}

// Given a rule, eager match the selector and return its specificity along with rule
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

// Find all matching rules in a stylesheet for a DOM element
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a StyleSheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

// Merge all stylesheet declarations as per specificity to generate element property map
fn element_properties(elem: &ElementData, stylesheet: &StyleSheet) -> PropertyMap {
    let mut matching_rules = matching_rules(elem, stylesheet);
    let mut properties = HashMap::new();

    matching_rules.sort_by(|a, b| a.0.cmp(&b.0));
    for (_, rule) in matching_rules.iter() {
        for declaration in rule.declarations.iter() {
            properties.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    properties
}

/// Walk through the entire DOM and apply styles to create a Style Tree
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a StyleSheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        css_props: match root.node_type {
            NodeType::Element(ref data) => element_properties(data, stylesheet),
            _ => HashMap::new()
        },
        children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect(),
    }
}
