use std::{any::Any, collections::HashMap, fmt::format, hash::Hash, };

use ast::*;
use span::Node;

use crate::{
    span::Span,
    visit::{visit_translation_unit, Visit},
};
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct Range {
    start: usize,
    end: usize,
}
impl Range {
    pub fn new<T: Into<usize>>(start: T, end: T) -> Range {
        Range {
            start: start.into(),
            end: end.into(),
        }
    }
}

macro_rules! range {
    ($s:expr, $e:expr) => {
        Range::new($s, $e)
    };
}

#[derive(Clone, Copy, Debug)]
pub struct RangeMap {
    from: Range,
    to: Range,
}

impl RangeMap {
    pub fn new(from: Range, to: Range) -> RangeMap {
        RangeMap { from, to }
    }
}

macro_rules! rmap {
    ($f:expr => $t:expr) => {
        RangeMap::new($f, $t)
    };
    (($f0:expr, $f1:expr) => ($t0:expr, $t1:expr)) => {
        RangeMap::new(range!($f0, $f1), range!($t0, $t1))
    };
    ($f0:expr, $f1:expr => $t0:expr, $t1:expr) => {
        RangeMap::new(range!($f0, $f1), range!($t0, $t1))
    };
}

#[derive(Clone, Debug)]
pub struct IndexMaps {
    ranges: Vec<RangeMap>,
}

impl IndexMaps {
    pub fn new(length: usize) -> IndexMaps {
        IndexMaps {
            ranges: vec![rmap!(0, length => 0, length)],
        }
    }

    pub fn map_index(&self, idx: usize) -> usize {
        for range in &self.ranges {
            if idx >= range.from.start && idx < range.from.end {
                return idx - range.from.start + range.to.start;
            }
        }
        panic!("Index outside range or in an unmapped region")
    }

    pub fn map_end_index(&self, idx: usize) -> usize {
        for range in &self.ranges {
            if idx >= range.from.start && idx <= range.from.end {
                return idx - range.from.start + range.to.start;
            }
        }
        panic!("Index outside range or in an unmapped region")
    }

    pub fn find_range_idx(&self, idx: usize) -> usize {
        for (i, range) in self.ranges.iter().enumerate() {
            if idx >= range.from.start && idx < range.from.end {
                return i;
            }
        }
        panic!("Index outside range or in an unmapped region")
    }

    pub fn find_range_end_idx(&self, idx: usize) -> usize {
        for (i, range) in self.ranges.iter().enumerate() {
            if idx >= range.from.start && idx <= range.from.end {
                return i;
            }
        }
        panic!("Index outside range or in an unmapped region")
    }

    pub fn shift_replace(&mut self, start: usize, end: usize, length: usize) {
        let starti = start as isize;
        let endi = end as isize;
        let lengthi = length as isize;

        let start_range_idx = self.find_range_idx(start);
        let end_range_idx = self.find_range_end_idx(end);

        if start_range_idx == end_range_idx {
            let delta_length = lengthi - (endi - starti);
            let mapped_start = self.map_index(start);
            let mapped_end = self.map_end_index(end);

            self.ranges
                .insert(start_range_idx + 1, self.ranges[end_range_idx].clone());

            {
                let start_range = &mut self.ranges[start_range_idx];
                start_range.from.end = start;
                start_range.to.end = mapped_start;
            }
            {
                let end_range = &mut self.ranges[end_range_idx + 1];
                end_range.from.start = end;
                end_range.to.start = (mapped_end as isize + delta_length) as usize;
                end_range.to.end = (end_range.to.end as isize + delta_length) as usize;

                for i in end_range_idx + 2..self.ranges.len() {
                    let range = &mut self.ranges[i];
                    range.to.start = (range.to.start as isize + delta_length) as usize;
                    range.to.end = (range.to.end as isize + delta_length) as usize;
                }
            }
        } else {
            assert!(end_range_idx > start_range_idx);

            //remove all maps inbetween
            let mut i = end_range_idx - 1;
            while i > start_range_idx {
                self.ranges.remove(start_range_idx + 1);
                i -= 1;
            }

            let end_range_idx = start_range_idx + 1;
            let mapped_start = self.map_index(start);
            let mapped_end = self.map_end_index(end);

            let delta_length = lengthi - (mapped_end as isize - mapped_start as isize);

            {
                let start_range = &mut self.ranges[start_range_idx];
                start_range.from.end = start;
                start_range.to.end = mapped_start;
            }
            {
                let end_range = &mut self.ranges[end_range_idx];
                end_range.from.start = end;
                end_range.to.start = (mapped_end as isize + delta_length) as usize;
                end_range.to.end = (end_range.to.end as isize + delta_length) as usize;

                for i in end_range_idx + 2..self.ranges.len() {
                    let range = &mut self.ranges[i];
                    range.to.start = (range.to.start as isize + delta_length) as usize;
                    range.to.end = (range.to.end as isize + delta_length) as usize;
                }
            }
        }
    }
}

impl Display for IndexMaps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for range in &self.ranges {
            write!(
                f,
                "{}, {} => {}, {}\n",
                range.from.start, range.from.end, range.to.start, range.to.end
            )?;
        }
        Ok(())
    }
}

pub struct IndexMappedString {
    string: String,
    index_maps: IndexMaps,
}

impl IndexMappedString {
    fn new(s: String) -> Self {
        IndexMappedString {
            index_maps: IndexMaps::new(s.len()),
            string: s,
        }
    }

    fn replace(&mut self, start: usize, end: usize, s: &str) {
        let length = s.len();
        let old_start_map = self.index_maps.map_index(start);
        let old_end_map = self.index_maps.map_end_index(end);
        self.index_maps.shift_replace(start, end, length);

        self.string.replace_range(old_start_map..old_end_map, s);
    }
}
pub struct Visiter {
    pub source_immut: String,
    pub offset: isize,
    pub source: IndexMappedString,
    pub enums: HashMap<String, HashMap<String, (Option<Node<TypeName>>, usize)>>,
}

impl Visiter {
    fn substr(&self, start: usize, end: usize) -> &str {
        let start = start as isize - 130;
        let end = end as isize - 130;

        let mapped_start = self.source.index_maps.map_index(start as usize);
        let mapped_end = self.source.index_maps.map_end_index(end as usize);


        &self.source.string[mapped_start as usize..mapped_end as usize]
    }

    fn source_substr(&self, start: usize, end: usize) -> &str {
        let start = start as isize - 130;
        let end = end as isize - 130;

        &self.source_immut[start as usize..end as usize]
    }

    fn str_replace(&mut self, start: usize, end: usize, s: &str) {
        let start = start as isize - 130 + self.offset as isize;
        let end = end as isize - 130 + self.offset as isize;

        // self.source = self.source[..start as usize].to_owned() + s + &self.source[end as usize..];
        // self.offset += s.len() as isize - (end - start).abs() as isize;

        self.source.replace(start as usize, end as usize, s);
    }
}

impl Visit<'_> for Visiter {
    fn visit_enum_type(&mut self, enum_type: &'_ EnumType, span: &'_ crate::span::Span) {
        let start = enum_type.identifier.clone().unwrap().clone().span.start;
        let ident = enum_type.identifier.clone().unwrap().clone().node.name;

        let is_classic_enum = !(enum_type.enumerators.iter().map(|v| v.node.inner_val.is_some()).filter(|v| *v).count() > 0);

        if is_classic_enum {
            return;
        }

        let mut variants = HashMap::new();

        let mut replace = format!(
            "
        union {}__u {{
        ",
            ident
        );

        let mut i = 0;
        let mut end = 0;
        for v in enum_type.enumerators.clone() {
            variants.insert(
                v.node.identifier.node.name.clone(),
                if let Some(node) = v.node.inner_val.clone() {
                    (Some(node.val), i)
                } else {
                    (None, i)
                },
            );
            i += 1;

            let tstr = self.substr(
                v.node.inner_val.clone().unwrap().val.span.start,
                v.node.inner_val.unwrap().val.span.end,
            );

            replace += &format!("{} {};\n", tstr, v.node.identifier.node.name);
            end = v.span.end;
        }
        replace += &format!(
            "\n}};

            struct {} {{
            int variant;
            union {}__u value;
        }};
        \n",
            ident, ident
        );

        self.str_replace(span.start, span.end, &replace);

        self.enums.insert(ident, variants);
    }

    fn visit_defer(&mut self, f: &'_ DeferStatement, span: &'_ Span) {
        let mut replace = format!("{{\n");

        self.visit_statement(&f.stmt.node, &f.stmt.span);
        replace += self.substr(f.stmt.span.start, f.stmt.span.end);

        replace += "\n//deferred statements\n";
        for s in &f.deferred {
            replace += self.substr(s.span.start, s.span.end);
        }

        replace += "\n}}";

        self.str_replace(span.start, span.end, &replace);
    }

    fn visit_for_each(&mut self, f: &'_ ForEachStatement, span: &'_ Span) {
        let t = &f.r.node.start.node;
        let t_str = "int";

        self.visit_statement(&f.s.node, &f.r.span);

        let v = self.substr(f.elt.span.start, f.elt.span.end);

        let mut replace = format!(
            "for ({} {}={};{} < {}; {}++) ",
            t_str,
            v,
            self.substr(f.r.node.start.span.start, f.r.node.start.span.end),
            v,
            self.substr(f.r.node.stop.span.start, f.r.node.stop.span.end),
            v
        );

        // self.visit_statement(&f.s.node, &f.r.span);
        replace += self.substr(f.s.span.start, f.s.span.end);

        self.str_replace(span.start, span.end, &replace);
    }

    fn visit_if_let(&mut self, f: &'_ IfLetStatement, span: &'_ Span) {
        if self.enums.contains_key(&f.sum_type.node.name) {
            self.visit_statement(&f.then_statement.node, &f.then_statement.span);

            let sum_type = self.enums.get(&f.sum_type.node.name).unwrap();
            let variant = sum_type
                .get(&f.variant.node.name)
                .expect("Invalid enum variant");

            let variant_type = variant.0.clone().unwrap();


            let mut replace = format!(
                "if ({} == {}.variant ) {{",
                variant.1, f.comparison.node.name
            );

            replace += &format!(
                "\n{} {} = {}.value.{};\n",
                self.source_substr(variant_type.span.start, variant_type.span.end),
                f.inner_val_binding.node.name,
                f.comparison.node.name,
                f.variant.node.name
            );

            replace += self.substr(f.then_statement.span.start, f.then_statement.span.end);

            replace += "}";

            if let Some(else_stmt) = &f.else_statement {
                self.visit_statement(&else_stmt.node, &else_stmt.span);
                replace += "else {";
                replace += self.substr(else_stmt.span.start, else_stmt.span.end);
                replace += "}";
            }

            self.str_replace(span.start, span.end, &replace);

            let ifs = f.then_statement.clone();
            // self.visit_statement(&ifs.node, &ifs.span);
        }
    }

    fn visit_binary_operator_expression(
        &mut self,
        binary_operator_expression: &'_ BinaryOperatorExpression,
        span: &'_ Span,
    ) {
        if let BinaryOperatorExpression {
            operator:
                Node {
                    node: BinaryOperator::Assign,
                    ..
                },
            lhs,
            rhs,
        } = binary_operator_expression
        {
            if let Expression::Identifier(ident) = &lhs.node {
                self.visit_expression(&rhs.node, &rhs.span);
                if let Expression::IfExpr(b) = &rhs.node {
                    if let IfExpr {
                        cond,
                        if_then,
                        if_ret,
                        else_stmts,
                        else_ret,
                        else_if,
                    } = &b.node
                    {
                        let mut replace =
                            format!("if ({}) {{ ", self.substr(cond.span.start, cond.span.end));
                        for stmt in if_then {
                            replace += self.substr(stmt.span.start, stmt.span.end);
                            replace += ";\n";
                        }
                        replace += &format!(
                            "{} = {};",
                            self.substr(ident.span.start, ident.span.end),
                            self.substr(if_ret.span.start, if_ret.span.end)
                        );
                        replace += "}";
                        for elses in else_if {
                            replace += &format!(
                                " else if ({}) {{",
                                self.substr(elses.node.cond.span.start, elses.node.cond.span.end)
                            );
                            for st in elses.node.if_then.clone() {
                                replace += self.substr(st.span.start, st.span.end);
                                replace += ";\n";
                            }
                            replace += &format!(
                                "{} = {};",
                                self.substr(ident.span.start, ident.span.end),
                                self.substr(
                                    elses.node.if_ret.span.start,
                                    elses.node.if_ret.span.end
                                )
                            );
                            replace += "}";
                        }
                        replace += &format!("else {{");

                        for stmts in else_stmts {
                            replace += self.substr(stmts.span.start, stmts.span.end);
                            replace += ";\n";
                        }
                        replace += &format!(
                            "{} = {};",
                            self.substr(ident.span.start, ident.span.end),
                            self.substr(else_ret.span.start, else_ret.span.end)
                        );
                        replace += "}";

                        self.str_replace(span.start, span.end, &replace);
                    }
                }
            }
        }
    }

    fn visit_if_expr(&mut self, if_expr: &'_ IfExpr, span: &'_ Span) {
        let mut replace = format!(
            "{} ? ",
            self.substr(if_expr.cond.span.start, if_expr.cond.span.end)
        );

        replace += "({";

        for s in if_expr.if_then.clone() {
            replace += self.substr(s.span.start, s.span.end);
            replace += ";";
        }

        replace += self.substr(if_expr.if_ret.span.start, if_expr.if_ret.span.end);
        replace += ";}) : ";

        for l in if_expr.else_if.clone() {
            replace += "(";
            replace += self.substr(l.node.cond.span.start, l.node.cond.span.end);
            replace += " ? ";
            replace += "({";
            for s in l.node.if_then {
                replace += self.substr(s.span.start, s.span.end);
                replace += ";";
            }
            replace += self.substr(l.node.if_ret.span.start, l.node.if_ret.span.end);
            replace += ";}) :";
        }

        replace += "({";
        for s in if_expr.else_stmts.clone() {
            replace += self.substr(s.span.start, s.span.end);
            replace += ";";
        }
        replace += self.substr(if_expr.else_ret.span.start, if_expr.else_ret.span.end);
        replace += ";})";

        for s in if_expr.else_if.clone() {
            replace += ")";
        }

        self.str_replace(span.start, span.end, &replace);
    }

    fn visit_match_block(&mut self, f: &'_ MatchStatement, span: &'_ Span) {
        let mut replace = format!("switch ({}) {{", self.substr(f.cond.span.start, f.cond.span.end));

        for case in &f.cases {
            replace += &format!("case {}: ",self.substr(case.node.cexpr.span.start, case.node.cexpr.span.end));
            if case.node.fall_through {
                replace += "{";
                for s in &case.node.stmts {
                    replace += self.substr(s.span.start, s.span.end);
                }
                replace += "break;}";
            } else {
                for s in &case.node.stmts {
                    replace += self.substr(s.span.start, s.span.end);
                }
            }
        }

        if let Some(default) = &f.default {
            replace += &format!("default: {}", self.substr(default.span.start, default.span.end));
        }

        replace += "}";
        self.str_replace(span.start, span.end, &replace);
    }

    fn visit_sum_type_builder(&mut self, a: &'_ SumTypeBuilder, s: &'_ Span) {
        if self.enums.contains_key(&a.sum_type.node.name) {
            let start = a.sum_type.span.start;
            let end = a.inner_val.span.end;
            let sum_type = self.enums.get(&a.sum_type.node.name).unwrap();
            let variant = sum_type
                .get(&a.variant.node.name)
                .expect("Invalid enum variant");

            let mut built = format!(
                "(struct {}){{
            .variant = {},
            .value = {{ .{} = {} }}
            }}",
               a.sum_type.node.name,
                variant.1,
                a.variant.node.name,
                self.substr(a.inner_val.span.start, a.inner_val.span.end)
            );

            let built = built.to_owned();

            //

            self.str_replace(s.start, s.end, &built);

        // self.source = self.source[..start].to_string() + &built + &self.source[end..];
        //
        // self.offset += built.len() as isize  - (end - start) as isize
        } else {
            // panic!("use of unknown sum type");
        }
    }
}

pub fn walk_ast(node: TranslationUnit, s: &str) {
    // for decl in node.0 {
    //     match decl.node {
    //         ExternalDeclaration::Declaration(node) => {
    //         if let Declaration { specifiers, declarators } = node.node {
    //
    //             if specifiers.len() == 1 {
    //                 if let DeclarationSpecifier::TypeSpecifier(Node{node: TypeSpecifier::Enum(e), span: _}) = &specifiers[0].node {
    //                     for vals in e.node.enumerators {
    //
    //                         }
    //                 }
    //             }
    //         }
    //         }
    //        _ => (),
    //     }
    //
    // }
    //
    let source = std::fs::read_to_string(s).unwrap();

    let mut visiter = Visiter {
        offset: 0,
        enums: HashMap::new(),
        source: IndexMappedString::new(source.clone()),
        source_immut: source,
    };


    visit_translation_unit(&mut visiter, &node);

    println!("{}", visiter.source.string);
}

// pub fn desugar_if_let(env: EnviIfLetStatement { variant, inner_val_binding, then_statement, else_statement }: IfLetStatement) -> Statement {
//
//
//
