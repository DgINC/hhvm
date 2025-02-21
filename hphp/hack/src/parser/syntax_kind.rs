/**
 * Copyright (c) 2016, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree. An additional
 * directory.
 *
 **
 *
 * THIS FILE IS @generated; DO NOT EDIT IT
 * To regenerate this file, run
 *
 *   buck run //hphp/hack/src:generate_full_fidelity
 *
 **
 *
 */

use ocamlrep::{FromOcamlRep, ToOcamlRep};

use crate::token_kind::TokenKind;

#[derive(Debug, Copy, Clone, FromOcamlRep, ToOcamlRep, PartialEq)]
pub enum SyntaxKind {
    Missing,
    Token(TokenKind),
    SyntaxList,
    EndOfFile,
    Script,
    QualifiedName,
    ModuleName,
    SimpleTypeSpecifier,
    LiteralExpression,
    PrefixedStringExpression,
    PrefixedCodeExpression,
    VariableExpression,
    PipeVariableExpression,
    FileAttributeSpecification,
    EnumDeclaration,
    EnumUse,
    Enumerator,
    EnumClassDeclaration,
    EnumClassEnumerator,
    AliasDeclaration,
    ContextAliasDeclaration,
    CaseTypeDeclaration,
    CaseTypeVariant,
    PropertyDeclaration,
    PropertyDeclarator,
    NamespaceDeclaration,
    NamespaceDeclarationHeader,
    NamespaceBody,
    NamespaceEmptyBody,
    NamespaceUseDeclaration,
    NamespaceGroupUseDeclaration,
    NamespaceUseClause,
    FunctionDeclaration,
    FunctionDeclarationHeader,
    Contexts,
    WhereClause,
    WhereConstraint,
    MethodishDeclaration,
    MethodishTraitResolution,
    ClassishDeclaration,
    ClassishBody,
    TraitUse,
    RequireClause,
    ConstDeclaration,
    ConstantDeclarator,
    TypeConstDeclaration,
    ContextConstDeclaration,
    DecoratedExpression,
    ParameterDeclaration,
    VariadicParameter,
    OldAttributeSpecification,
    AttributeSpecification,
    Attribute,
    InclusionExpression,
    InclusionDirective,
    CompoundStatement,
    ExpressionStatement,
    MarkupSection,
    MarkupSuffix,
    UnsetStatement,
    DeclareLocalStatement,
    UsingStatementBlockScoped,
    UsingStatementFunctionScoped,
    WhileStatement,
    IfStatement,
    ElseClause,
    TryStatement,
    CatchClause,
    FinallyClause,
    DoStatement,
    ForStatement,
    ForeachStatement,
    SwitchStatement,
    SwitchSection,
    SwitchFallthrough,
    CaseLabel,
    DefaultLabel,
    MatchStatement,
    MatchStatementArm,
    ReturnStatement,
    YieldBreakStatement,
    ThrowStatement,
    BreakStatement,
    ContinueStatement,
    EchoStatement,
    ConcurrentStatement,
    SimpleInitializer,
    AnonymousClass,
    AnonymousFunction,
    AnonymousFunctionUseClause,
    VariablePattern,
    ConstructorPattern,
    RefinementPattern,
    LambdaExpression,
    LambdaSignature,
    CastExpression,
    ScopeResolutionExpression,
    MemberSelectionExpression,
    SafeMemberSelectionExpression,
    EmbeddedMemberSelectionExpression,
    YieldExpression,
    PrefixUnaryExpression,
    PostfixUnaryExpression,
    BinaryExpression,
    IsExpression,
    AsExpression,
    NullableAsExpression,
    UpcastExpression,
    ConditionalExpression,
    EvalExpression,
    IssetExpression,
    FunctionCallExpression,
    FunctionPointerExpression,
    ParenthesizedExpression,
    BracedExpression,
    ETSpliceExpression,
    EmbeddedBracedExpression,
    ListExpression,
    CollectionLiteralExpression,
    ObjectCreationExpression,
    ConstructorCall,
    DarrayIntrinsicExpression,
    DictionaryIntrinsicExpression,
    KeysetIntrinsicExpression,
    VarrayIntrinsicExpression,
    VectorIntrinsicExpression,
    ElementInitializer,
    SubscriptExpression,
    EmbeddedSubscriptExpression,
    AwaitableCreationExpression,
    XHPChildrenDeclaration,
    XHPChildrenParenthesizedList,
    XHPCategoryDeclaration,
    XHPEnumType,
    XHPLateinit,
    XHPRequired,
    XHPClassAttributeDeclaration,
    XHPClassAttribute,
    XHPSimpleClassAttribute,
    XHPSimpleAttribute,
    XHPSpreadAttribute,
    XHPOpen,
    XHPExpression,
    XHPClose,
    TypeConstant,
    VectorTypeSpecifier,
    KeysetTypeSpecifier,
    TupleTypeExplicitSpecifier,
    VarrayTypeSpecifier,
    FunctionCtxTypeSpecifier,
    TypeParameter,
    TypeConstraint,
    ContextConstraint,
    DarrayTypeSpecifier,
    DictionaryTypeSpecifier,
    ClosureTypeSpecifier,
    ClosureParameterTypeSpecifier,
    TypeRefinement,
    TypeInRefinement,
    CtxInRefinement,
    ClassnameTypeSpecifier,
    FieldSpecifier,
    FieldInitializer,
    ShapeTypeSpecifier,
    ShapeExpression,
    TupleExpression,
    GenericTypeSpecifier,
    NullableTypeSpecifier,
    LikeTypeSpecifier,
    SoftTypeSpecifier,
    AttributizedSpecifier,
    ReifiedTypeArgument,
    TypeArguments,
    TypeParameters,
    TupleTypeSpecifier,
    UnionTypeSpecifier,
    IntersectionTypeSpecifier,
    ErrorSyntax,
    ListItem,
    EnumClassLabelExpression,
    ModuleDeclaration,
    ModuleExports,
    ModuleImports,
    ModuleMembershipDeclaration,
    PackageExpression,

}

impl SyntaxKind {
    pub fn to_string(&self) -> &str {
        match self {
            SyntaxKind::SyntaxList => "list",
            SyntaxKind::Missing => "missing",
            SyntaxKind::Token(_) => "token",
            SyntaxKind::EndOfFile                         => "end_of_file",
            SyntaxKind::Script                            => "script",
            SyntaxKind::QualifiedName                     => "qualified_name",
            SyntaxKind::ModuleName                        => "module_name",
            SyntaxKind::SimpleTypeSpecifier               => "simple_type_specifier",
            SyntaxKind::LiteralExpression                 => "literal",
            SyntaxKind::PrefixedStringExpression          => "prefixed_string",
            SyntaxKind::PrefixedCodeExpression            => "prefixed_code",
            SyntaxKind::VariableExpression                => "variable",
            SyntaxKind::PipeVariableExpression            => "pipe_variable",
            SyntaxKind::FileAttributeSpecification        => "file_attribute_specification",
            SyntaxKind::EnumDeclaration                   => "enum_declaration",
            SyntaxKind::EnumUse                           => "enum_use",
            SyntaxKind::Enumerator                        => "enumerator",
            SyntaxKind::EnumClassDeclaration              => "enum_class_declaration",
            SyntaxKind::EnumClassEnumerator               => "enum_class_enumerator",
            SyntaxKind::AliasDeclaration                  => "alias_declaration",
            SyntaxKind::ContextAliasDeclaration           => "context_alias_declaration",
            SyntaxKind::CaseTypeDeclaration               => "case_type_declaration",
            SyntaxKind::CaseTypeVariant                   => "case_type_variant",
            SyntaxKind::PropertyDeclaration               => "property_declaration",
            SyntaxKind::PropertyDeclarator                => "property_declarator",
            SyntaxKind::NamespaceDeclaration              => "namespace_declaration",
            SyntaxKind::NamespaceDeclarationHeader        => "namespace_declaration_header",
            SyntaxKind::NamespaceBody                     => "namespace_body",
            SyntaxKind::NamespaceEmptyBody                => "namespace_empty_body",
            SyntaxKind::NamespaceUseDeclaration           => "namespace_use_declaration",
            SyntaxKind::NamespaceGroupUseDeclaration      => "namespace_group_use_declaration",
            SyntaxKind::NamespaceUseClause                => "namespace_use_clause",
            SyntaxKind::FunctionDeclaration               => "function_declaration",
            SyntaxKind::FunctionDeclarationHeader         => "function_declaration_header",
            SyntaxKind::Contexts                          => "contexts",
            SyntaxKind::WhereClause                       => "where_clause",
            SyntaxKind::WhereConstraint                   => "where_constraint",
            SyntaxKind::MethodishDeclaration              => "methodish_declaration",
            SyntaxKind::MethodishTraitResolution          => "methodish_trait_resolution",
            SyntaxKind::ClassishDeclaration               => "classish_declaration",
            SyntaxKind::ClassishBody                      => "classish_body",
            SyntaxKind::TraitUse                          => "trait_use",
            SyntaxKind::RequireClause                     => "require_clause",
            SyntaxKind::ConstDeclaration                  => "const_declaration",
            SyntaxKind::ConstantDeclarator                => "constant_declarator",
            SyntaxKind::TypeConstDeclaration              => "type_const_declaration",
            SyntaxKind::ContextConstDeclaration           => "context_const_declaration",
            SyntaxKind::DecoratedExpression               => "decorated_expression",
            SyntaxKind::ParameterDeclaration              => "parameter_declaration",
            SyntaxKind::VariadicParameter                 => "variadic_parameter",
            SyntaxKind::OldAttributeSpecification         => "old_attribute_specification",
            SyntaxKind::AttributeSpecification            => "attribute_specification",
            SyntaxKind::Attribute                         => "attribute",
            SyntaxKind::InclusionExpression               => "inclusion_expression",
            SyntaxKind::InclusionDirective                => "inclusion_directive",
            SyntaxKind::CompoundStatement                 => "compound_statement",
            SyntaxKind::ExpressionStatement               => "expression_statement",
            SyntaxKind::MarkupSection                     => "markup_section",
            SyntaxKind::MarkupSuffix                      => "markup_suffix",
            SyntaxKind::UnsetStatement                    => "unset_statement",
            SyntaxKind::DeclareLocalStatement             => "declare_local_statement",
            SyntaxKind::UsingStatementBlockScoped         => "using_statement_block_scoped",
            SyntaxKind::UsingStatementFunctionScoped      => "using_statement_function_scoped",
            SyntaxKind::WhileStatement                    => "while_statement",
            SyntaxKind::IfStatement                       => "if_statement",
            SyntaxKind::ElseClause                        => "else_clause",
            SyntaxKind::TryStatement                      => "try_statement",
            SyntaxKind::CatchClause                       => "catch_clause",
            SyntaxKind::FinallyClause                     => "finally_clause",
            SyntaxKind::DoStatement                       => "do_statement",
            SyntaxKind::ForStatement                      => "for_statement",
            SyntaxKind::ForeachStatement                  => "foreach_statement",
            SyntaxKind::SwitchStatement                   => "switch_statement",
            SyntaxKind::SwitchSection                     => "switch_section",
            SyntaxKind::SwitchFallthrough                 => "switch_fallthrough",
            SyntaxKind::CaseLabel                         => "case_label",
            SyntaxKind::DefaultLabel                      => "default_label",
            SyntaxKind::MatchStatement                    => "match_statement",
            SyntaxKind::MatchStatementArm                 => "match_statement_arm",
            SyntaxKind::ReturnStatement                   => "return_statement",
            SyntaxKind::YieldBreakStatement               => "yield_break_statement",
            SyntaxKind::ThrowStatement                    => "throw_statement",
            SyntaxKind::BreakStatement                    => "break_statement",
            SyntaxKind::ContinueStatement                 => "continue_statement",
            SyntaxKind::EchoStatement                     => "echo_statement",
            SyntaxKind::ConcurrentStatement               => "concurrent_statement",
            SyntaxKind::SimpleInitializer                 => "simple_initializer",
            SyntaxKind::AnonymousClass                    => "anonymous_class",
            SyntaxKind::AnonymousFunction                 => "anonymous_function",
            SyntaxKind::AnonymousFunctionUseClause        => "anonymous_function_use_clause",
            SyntaxKind::VariablePattern                   => "variable_pattern",
            SyntaxKind::ConstructorPattern                => "constructor_pattern",
            SyntaxKind::RefinementPattern                 => "refinement_pattern",
            SyntaxKind::LambdaExpression                  => "lambda_expression",
            SyntaxKind::LambdaSignature                   => "lambda_signature",
            SyntaxKind::CastExpression                    => "cast_expression",
            SyntaxKind::ScopeResolutionExpression         => "scope_resolution_expression",
            SyntaxKind::MemberSelectionExpression         => "member_selection_expression",
            SyntaxKind::SafeMemberSelectionExpression     => "safe_member_selection_expression",
            SyntaxKind::EmbeddedMemberSelectionExpression => "embedded_member_selection_expression",
            SyntaxKind::YieldExpression                   => "yield_expression",
            SyntaxKind::PrefixUnaryExpression             => "prefix_unary_expression",
            SyntaxKind::PostfixUnaryExpression            => "postfix_unary_expression",
            SyntaxKind::BinaryExpression                  => "binary_expression",
            SyntaxKind::IsExpression                      => "is_expression",
            SyntaxKind::AsExpression                      => "as_expression",
            SyntaxKind::NullableAsExpression              => "nullable_as_expression",
            SyntaxKind::UpcastExpression                  => "upcast_expression",
            SyntaxKind::ConditionalExpression             => "conditional_expression",
            SyntaxKind::EvalExpression                    => "eval_expression",
            SyntaxKind::IssetExpression                   => "isset_expression",
            SyntaxKind::FunctionCallExpression            => "function_call_expression",
            SyntaxKind::FunctionPointerExpression         => "function_pointer_expression",
            SyntaxKind::ParenthesizedExpression           => "parenthesized_expression",
            SyntaxKind::BracedExpression                  => "braced_expression",
            SyntaxKind::ETSpliceExpression                => "et_splice_expression",
            SyntaxKind::EmbeddedBracedExpression          => "embedded_braced_expression",
            SyntaxKind::ListExpression                    => "list_expression",
            SyntaxKind::CollectionLiteralExpression       => "collection_literal_expression",
            SyntaxKind::ObjectCreationExpression          => "object_creation_expression",
            SyntaxKind::ConstructorCall                   => "constructor_call",
            SyntaxKind::DarrayIntrinsicExpression         => "darray_intrinsic_expression",
            SyntaxKind::DictionaryIntrinsicExpression     => "dictionary_intrinsic_expression",
            SyntaxKind::KeysetIntrinsicExpression         => "keyset_intrinsic_expression",
            SyntaxKind::VarrayIntrinsicExpression         => "varray_intrinsic_expression",
            SyntaxKind::VectorIntrinsicExpression         => "vector_intrinsic_expression",
            SyntaxKind::ElementInitializer                => "element_initializer",
            SyntaxKind::SubscriptExpression               => "subscript_expression",
            SyntaxKind::EmbeddedSubscriptExpression       => "embedded_subscript_expression",
            SyntaxKind::AwaitableCreationExpression       => "awaitable_creation_expression",
            SyntaxKind::XHPChildrenDeclaration            => "xhp_children_declaration",
            SyntaxKind::XHPChildrenParenthesizedList      => "xhp_children_parenthesized_list",
            SyntaxKind::XHPCategoryDeclaration            => "xhp_category_declaration",
            SyntaxKind::XHPEnumType                       => "xhp_enum_type",
            SyntaxKind::XHPLateinit                       => "xhp_lateinit",
            SyntaxKind::XHPRequired                       => "xhp_required",
            SyntaxKind::XHPClassAttributeDeclaration      => "xhp_class_attribute_declaration",
            SyntaxKind::XHPClassAttribute                 => "xhp_class_attribute",
            SyntaxKind::XHPSimpleClassAttribute           => "xhp_simple_class_attribute",
            SyntaxKind::XHPSimpleAttribute                => "xhp_simple_attribute",
            SyntaxKind::XHPSpreadAttribute                => "xhp_spread_attribute",
            SyntaxKind::XHPOpen                           => "xhp_open",
            SyntaxKind::XHPExpression                     => "xhp_expression",
            SyntaxKind::XHPClose                          => "xhp_close",
            SyntaxKind::TypeConstant                      => "type_constant",
            SyntaxKind::VectorTypeSpecifier               => "vector_type_specifier",
            SyntaxKind::KeysetTypeSpecifier               => "keyset_type_specifier",
            SyntaxKind::TupleTypeExplicitSpecifier        => "tuple_type_explicit_specifier",
            SyntaxKind::VarrayTypeSpecifier               => "varray_type_specifier",
            SyntaxKind::FunctionCtxTypeSpecifier          => "function_ctx_type_specifier",
            SyntaxKind::TypeParameter                     => "type_parameter",
            SyntaxKind::TypeConstraint                    => "type_constraint",
            SyntaxKind::ContextConstraint                 => "context_constraint",
            SyntaxKind::DarrayTypeSpecifier               => "darray_type_specifier",
            SyntaxKind::DictionaryTypeSpecifier           => "dictionary_type_specifier",
            SyntaxKind::ClosureTypeSpecifier              => "closure_type_specifier",
            SyntaxKind::ClosureParameterTypeSpecifier     => "closure_parameter_type_specifier",
            SyntaxKind::TypeRefinement                    => "type_refinement",
            SyntaxKind::TypeInRefinement                  => "type_in_refinement",
            SyntaxKind::CtxInRefinement                   => "ctx_in_refinement",
            SyntaxKind::ClassnameTypeSpecifier            => "classname_type_specifier",
            SyntaxKind::FieldSpecifier                    => "field_specifier",
            SyntaxKind::FieldInitializer                  => "field_initializer",
            SyntaxKind::ShapeTypeSpecifier                => "shape_type_specifier",
            SyntaxKind::ShapeExpression                   => "shape_expression",
            SyntaxKind::TupleExpression                   => "tuple_expression",
            SyntaxKind::GenericTypeSpecifier              => "generic_type_specifier",
            SyntaxKind::NullableTypeSpecifier             => "nullable_type_specifier",
            SyntaxKind::LikeTypeSpecifier                 => "like_type_specifier",
            SyntaxKind::SoftTypeSpecifier                 => "soft_type_specifier",
            SyntaxKind::AttributizedSpecifier             => "attributized_specifier",
            SyntaxKind::ReifiedTypeArgument               => "reified_type_argument",
            SyntaxKind::TypeArguments                     => "type_arguments",
            SyntaxKind::TypeParameters                    => "type_parameters",
            SyntaxKind::TupleTypeSpecifier                => "tuple_type_specifier",
            SyntaxKind::UnionTypeSpecifier                => "union_type_specifier",
            SyntaxKind::IntersectionTypeSpecifier         => "intersection_type_specifier",
            SyntaxKind::ErrorSyntax                       => "error",
            SyntaxKind::ListItem                          => "list_item",
            SyntaxKind::EnumClassLabelExpression          => "enum_class_label",
            SyntaxKind::ModuleDeclaration                 => "module_declaration",
            SyntaxKind::ModuleExports                     => "module_exports",
            SyntaxKind::ModuleImports                     => "module_imports",
            SyntaxKind::ModuleMembershipDeclaration       => "module_membership_declaration",
            SyntaxKind::PackageExpression                 => "package_expression",
        }
    }

    pub fn ocaml_tag(self) -> u8 {
        match self {
            SyntaxKind::Missing => 0,
            SyntaxKind::Token(_) => 0,
            SyntaxKind::SyntaxList => 1,
            SyntaxKind::EndOfFile => 2,
            SyntaxKind::Script => 3,
            SyntaxKind::QualifiedName => 4,
            SyntaxKind::ModuleName => 5,
            SyntaxKind::SimpleTypeSpecifier => 6,
            SyntaxKind::LiteralExpression => 7,
            SyntaxKind::PrefixedStringExpression => 8,
            SyntaxKind::PrefixedCodeExpression => 9,
            SyntaxKind::VariableExpression => 10,
            SyntaxKind::PipeVariableExpression => 11,
            SyntaxKind::FileAttributeSpecification => 12,
            SyntaxKind::EnumDeclaration => 13,
            SyntaxKind::EnumUse => 14,
            SyntaxKind::Enumerator => 15,
            SyntaxKind::EnumClassDeclaration => 16,
            SyntaxKind::EnumClassEnumerator => 17,
            SyntaxKind::AliasDeclaration => 18,
            SyntaxKind::ContextAliasDeclaration => 19,
            SyntaxKind::CaseTypeDeclaration => 20,
            SyntaxKind::CaseTypeVariant => 21,
            SyntaxKind::PropertyDeclaration => 22,
            SyntaxKind::PropertyDeclarator => 23,
            SyntaxKind::NamespaceDeclaration => 24,
            SyntaxKind::NamespaceDeclarationHeader => 25,
            SyntaxKind::NamespaceBody => 26,
            SyntaxKind::NamespaceEmptyBody => 27,
            SyntaxKind::NamespaceUseDeclaration => 28,
            SyntaxKind::NamespaceGroupUseDeclaration => 29,
            SyntaxKind::NamespaceUseClause => 30,
            SyntaxKind::FunctionDeclaration => 31,
            SyntaxKind::FunctionDeclarationHeader => 32,
            SyntaxKind::Contexts => 33,
            SyntaxKind::WhereClause => 34,
            SyntaxKind::WhereConstraint => 35,
            SyntaxKind::MethodishDeclaration => 36,
            SyntaxKind::MethodishTraitResolution => 37,
            SyntaxKind::ClassishDeclaration => 38,
            SyntaxKind::ClassishBody => 39,
            SyntaxKind::TraitUse => 40,
            SyntaxKind::RequireClause => 41,
            SyntaxKind::ConstDeclaration => 42,
            SyntaxKind::ConstantDeclarator => 43,
            SyntaxKind::TypeConstDeclaration => 44,
            SyntaxKind::ContextConstDeclaration => 45,
            SyntaxKind::DecoratedExpression => 46,
            SyntaxKind::ParameterDeclaration => 47,
            SyntaxKind::VariadicParameter => 48,
            SyntaxKind::OldAttributeSpecification => 49,
            SyntaxKind::AttributeSpecification => 50,
            SyntaxKind::Attribute => 51,
            SyntaxKind::InclusionExpression => 52,
            SyntaxKind::InclusionDirective => 53,
            SyntaxKind::CompoundStatement => 54,
            SyntaxKind::ExpressionStatement => 55,
            SyntaxKind::MarkupSection => 56,
            SyntaxKind::MarkupSuffix => 57,
            SyntaxKind::UnsetStatement => 58,
            SyntaxKind::DeclareLocalStatement => 59,
            SyntaxKind::UsingStatementBlockScoped => 60,
            SyntaxKind::UsingStatementFunctionScoped => 61,
            SyntaxKind::WhileStatement => 62,
            SyntaxKind::IfStatement => 63,
            SyntaxKind::ElseClause => 64,
            SyntaxKind::TryStatement => 65,
            SyntaxKind::CatchClause => 66,
            SyntaxKind::FinallyClause => 67,
            SyntaxKind::DoStatement => 68,
            SyntaxKind::ForStatement => 69,
            SyntaxKind::ForeachStatement => 70,
            SyntaxKind::SwitchStatement => 71,
            SyntaxKind::SwitchSection => 72,
            SyntaxKind::SwitchFallthrough => 73,
            SyntaxKind::CaseLabel => 74,
            SyntaxKind::DefaultLabel => 75,
            SyntaxKind::MatchStatement => 76,
            SyntaxKind::MatchStatementArm => 77,
            SyntaxKind::ReturnStatement => 78,
            SyntaxKind::YieldBreakStatement => 79,
            SyntaxKind::ThrowStatement => 80,
            SyntaxKind::BreakStatement => 81,
            SyntaxKind::ContinueStatement => 82,
            SyntaxKind::EchoStatement => 83,
            SyntaxKind::ConcurrentStatement => 84,
            SyntaxKind::SimpleInitializer => 85,
            SyntaxKind::AnonymousClass => 86,
            SyntaxKind::AnonymousFunction => 87,
            SyntaxKind::AnonymousFunctionUseClause => 88,
            SyntaxKind::VariablePattern => 89,
            SyntaxKind::ConstructorPattern => 90,
            SyntaxKind::RefinementPattern => 91,
            SyntaxKind::LambdaExpression => 92,
            SyntaxKind::LambdaSignature => 93,
            SyntaxKind::CastExpression => 94,
            SyntaxKind::ScopeResolutionExpression => 95,
            SyntaxKind::MemberSelectionExpression => 96,
            SyntaxKind::SafeMemberSelectionExpression => 97,
            SyntaxKind::EmbeddedMemberSelectionExpression => 98,
            SyntaxKind::YieldExpression => 99,
            SyntaxKind::PrefixUnaryExpression => 100,
            SyntaxKind::PostfixUnaryExpression => 101,
            SyntaxKind::BinaryExpression => 102,
            SyntaxKind::IsExpression => 103,
            SyntaxKind::AsExpression => 104,
            SyntaxKind::NullableAsExpression => 105,
            SyntaxKind::UpcastExpression => 106,
            SyntaxKind::ConditionalExpression => 107,
            SyntaxKind::EvalExpression => 108,
            SyntaxKind::IssetExpression => 109,
            SyntaxKind::FunctionCallExpression => 110,
            SyntaxKind::FunctionPointerExpression => 111,
            SyntaxKind::ParenthesizedExpression => 112,
            SyntaxKind::BracedExpression => 113,
            SyntaxKind::ETSpliceExpression => 114,
            SyntaxKind::EmbeddedBracedExpression => 115,
            SyntaxKind::ListExpression => 116,
            SyntaxKind::CollectionLiteralExpression => 117,
            SyntaxKind::ObjectCreationExpression => 118,
            SyntaxKind::ConstructorCall => 119,
            SyntaxKind::DarrayIntrinsicExpression => 120,
            SyntaxKind::DictionaryIntrinsicExpression => 121,
            SyntaxKind::KeysetIntrinsicExpression => 122,
            SyntaxKind::VarrayIntrinsicExpression => 123,
            SyntaxKind::VectorIntrinsicExpression => 124,
            SyntaxKind::ElementInitializer => 125,
            SyntaxKind::SubscriptExpression => 126,
            SyntaxKind::EmbeddedSubscriptExpression => 127,
            SyntaxKind::AwaitableCreationExpression => 128,
            SyntaxKind::XHPChildrenDeclaration => 129,
            SyntaxKind::XHPChildrenParenthesizedList => 130,
            SyntaxKind::XHPCategoryDeclaration => 131,
            SyntaxKind::XHPEnumType => 132,
            SyntaxKind::XHPLateinit => 133,
            SyntaxKind::XHPRequired => 134,
            SyntaxKind::XHPClassAttributeDeclaration => 135,
            SyntaxKind::XHPClassAttribute => 136,
            SyntaxKind::XHPSimpleClassAttribute => 137,
            SyntaxKind::XHPSimpleAttribute => 138,
            SyntaxKind::XHPSpreadAttribute => 139,
            SyntaxKind::XHPOpen => 140,
            SyntaxKind::XHPExpression => 141,
            SyntaxKind::XHPClose => 142,
            SyntaxKind::TypeConstant => 143,
            SyntaxKind::VectorTypeSpecifier => 144,
            SyntaxKind::KeysetTypeSpecifier => 145,
            SyntaxKind::TupleTypeExplicitSpecifier => 146,
            SyntaxKind::VarrayTypeSpecifier => 147,
            SyntaxKind::FunctionCtxTypeSpecifier => 148,
            SyntaxKind::TypeParameter => 149,
            SyntaxKind::TypeConstraint => 150,
            SyntaxKind::ContextConstraint => 151,
            SyntaxKind::DarrayTypeSpecifier => 152,
            SyntaxKind::DictionaryTypeSpecifier => 153,
            SyntaxKind::ClosureTypeSpecifier => 154,
            SyntaxKind::ClosureParameterTypeSpecifier => 155,
            SyntaxKind::TypeRefinement => 156,
            SyntaxKind::TypeInRefinement => 157,
            SyntaxKind::CtxInRefinement => 158,
            SyntaxKind::ClassnameTypeSpecifier => 159,
            SyntaxKind::FieldSpecifier => 160,
            SyntaxKind::FieldInitializer => 161,
            SyntaxKind::ShapeTypeSpecifier => 162,
            SyntaxKind::ShapeExpression => 163,
            SyntaxKind::TupleExpression => 164,
            SyntaxKind::GenericTypeSpecifier => 165,
            SyntaxKind::NullableTypeSpecifier => 166,
            SyntaxKind::LikeTypeSpecifier => 167,
            SyntaxKind::SoftTypeSpecifier => 168,
            SyntaxKind::AttributizedSpecifier => 169,
            SyntaxKind::ReifiedTypeArgument => 170,
            SyntaxKind::TypeArguments => 171,
            SyntaxKind::TypeParameters => 172,
            SyntaxKind::TupleTypeSpecifier => 173,
            SyntaxKind::UnionTypeSpecifier => 174,
            SyntaxKind::IntersectionTypeSpecifier => 175,
            SyntaxKind::ErrorSyntax => 176,
            SyntaxKind::ListItem => 177,
            SyntaxKind::EnumClassLabelExpression => 178,
            SyntaxKind::ModuleDeclaration => 179,
            SyntaxKind::ModuleExports => 180,
            SyntaxKind::ModuleImports => 181,
            SyntaxKind::ModuleMembershipDeclaration => 182,
            SyntaxKind::PackageExpression => 183,
        }
    }
}
