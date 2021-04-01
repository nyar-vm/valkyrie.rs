ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "f",
                    span: 21..21,
                },
            ],
        },
    ),
    monadic: false,
    caller: Normal,
    arguments: Some(
        ApplyCallTerms {
            terms: [],
            span: 22..23,
        },
    ),
    trailing: None,
    span: 22..23,
}
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "f",
                    span: 27..27,
                },
            ],
        },
    ),
    monadic: false,
    caller: Normal,
    arguments: Some(
        ApplyCallTerms {
            terms: [
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: None,
                    body: Number(
                        NumberLiteralNode {
                            value: "1",
                            unit: None,
                            span: 29..29,
                        },
                    ),
                    span: 29..29,
                },
            ],
            span: 28..30,
        },
    ),
    trailing: None,
    span: 28..30,
}
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "f",
                    span: 34..34,
                },
            ],
        },
    ),
    monadic: false,
    caller: Normal,
    arguments: Some(
        ApplyCallTerms {
            terms: [
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: Some(
                        IdentifierNode {
                            name: "k",
                            span: 36..36,
                        },
                    ),
                    body: Number(
                        NumberLiteralNode {
                            value: "1",
                            unit: None,
                            span: 39..39,
                        },
                    ),
                    span: 36..39,
                },
            ],
            span: 35..40,
        },
    ),
    trailing: None,
    span: 35..40,
}
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "f",
                    span: 44..44,
                },
            ],
        },
    ),
    monadic: false,
    caller: Normal,
    arguments: Some(
        ApplyCallTerms {
            terms: [
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: None,
                    body: Null(
                        NullNode {
                            nil: false,
                            span: 46..49,
                        },
                    ),
                    span: 46..49,
                },
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: Some(
                        IdentifierNode {
                            name: "a",
                            span: 52..52,
                        },
                    ),
                    body: Number(
                        NumberLiteralNode {
                            value: "1",
                            unit: None,
                            span: 55..55,
                        },
                    ),
                    span: 52..55,
                },
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: None,
                    body: Prefix(
                        PrefixNode {
                            operator: OperatorNode {
                                kind: Unpack {
                                    level: 2,
                                },
                                span: 58..59,
                            },
                            base: Symbol(
                                NamePathNode {
                                    names: [
                                        IdentifierNode {
                                            name: "args",
                                            span: 60..63,
                                        },
                                    ],
                                },
                            ),
                            span: 58..63,
                        },
                    ),
                    span: 58..63,
                },
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: None,
                    body: Prefix(
                        PrefixNode {
                            operator: OperatorNode {
                                kind: Unpack {
                                    level: 3,
                                },
                                span: 66..68,
                            },
                            base: Symbol(
                                NamePathNode {
                                    names: [
                                        IdentifierNode {
                                            name: "kwargs",
                                            span: 69..74,
                                        },
                                    ],
                                },
                            ),
                            span: 66..74,
                        },
                    ),
                    span: 66..74,
                },
            ],
            span: 45..75,
        },
    ),
    trailing: None,
    span: 45..75,
}
ClosureCallNode {
    base: LambdaCall(
        ClosureCallNode {
            base: LambdaCall(
                ClosureCallNode {
                    base: LambdaCall(
                        ClosureCallNode {
                            base: Symbol(
                                NamePathNode {
                                    names: [
                                        IdentifierNode {
                                            name: "f",
                                            span: 99..99,
                                        },
                                    ],
                                },
                            ),
                            monadic: false,
                            caller: Normal,
                            arguments: Some(
                                ApplyCallTerms {
                                    terms: [],
                                    span: 100..101,
                                },
                            ),
                            trailing: None,
                            span: 100..101,
                        },
                    ),
                    monadic: false,
                    caller: Normal,
                    arguments: Some(
                        ApplyCallTerms {
                            terms: [
                                ApplyCallItem {
                                    modifiers: ModifiersNode {
                                        terms: [],
                                    },
                                    parameter: None,
                                    body: Number(
                                        NumberLiteralNode {
                                            value: "1",
                                            unit: None,
                                            span: 103..103,
                                        },
                                    ),
                                    span: 103..103,
                                },
                            ],
                            span: 102..104,
                        },
                    ),
                    trailing: None,
                    span: 102..104,
                },
            ),
            monadic: false,
            caller: Normal,
            arguments: Some(
                ApplyCallTerms {
                    terms: [
                        ApplyCallItem {
                            modifiers: ModifiersNode {
                                terms: [],
                            },
                            parameter: None,
                            body: Number(
                                NumberLiteralNode {
                                    value: "1",
                                    unit: None,
                                    span: 106..106,
                                },
                            ),
                            span: 106..106,
                        },
                        ApplyCallItem {
                            modifiers: ModifiersNode {
                                terms: [],
                            },
                            parameter: None,
                            body: Number(
                                NumberLiteralNode {
                                    value: "2",
                                    unit: None,
                                    span: 109..109,
                                },
                            ),
                            span: 109..109,
                        },
                    ],
                    span: 105..110,
                },
            ),
            trailing: None,
            span: 105..110,
        },
    ),
    monadic: false,
    caller: Normal,
    arguments: Some(
        ApplyCallTerms {
            terms: [
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: None,
                    body: Number(
                        NumberLiteralNode {
                            value: "1",
                            unit: None,
                            span: 112..112,
                        },
                    ),
                    span: 112..112,
                },
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: None,
                    body: Number(
                        NumberLiteralNode {
                            value: "2",
                            unit: None,
                            span: 115..115,
                        },
                    ),
                    span: 115..115,
                },
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: None,
                    body: Number(
                        NumberLiteralNode {
                            value: "3",
                            unit: None,
                            span: 118..118,
                        },
                    ),
                    span: 118..118,
                },
            ],
            span: 111..119,
        },
    ),
    trailing: None,
    span: 111..119,
}
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "a",
                    span: 138..138,
                },
            ],
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "b",
            span: 140..140,
        },
    ),
    arguments: None,
    trailing: None,
    span: 139..140,
}
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "a",
                    span: 144..144,
                },
            ],
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "b",
            span: 146..146,
        },
    ),
    arguments: Some(
        ApplyCallTerms {
            terms: [],
            span: 147..148,
        },
    ),
    trailing: None,
    span: 145..148,
}
ClosureCallNode {
    base: LambdaCall(
        ClosureCallNode {
            base: Symbol(
                NamePathNode {
                    names: [
                        IdentifierNode {
                            name: "a",
                            span: 152..152,
                        },
                    ],
                },
            ),
            monadic: false,
            caller: Internal(
                IdentifierNode {
                    name: "b",
                    span: 154..154,
                },
            ),
            arguments: None,
            trailing: None,
            span: 153..154,
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "c",
            span: 156..156,
        },
    ),
    arguments: Some(
        ApplyCallTerms {
            terms: [],
            span: 157..158,
        },
    ),
    trailing: None,
    span: 155..158,
}
ClosureCallNode {
    base: LambdaCall(
        ClosureCallNode {
            base: LambdaCall(
                ClosureCallNode {
                    base: LambdaCall(
                        ClosureCallNode {
                            base: LambdaCall(
                                ClosureCallNode {
                                    base: LambdaCall(
                                        ClosureCallNode {
                                            base: Symbol(
                                                NamePathNode {
                                                    names: [
                                                        IdentifierNode {
                                                            name: "a",
                                                            span: 162..162,
                                                        },
                                                    ],
                                                },
                                            ),
                                            monadic: false,
                                            caller: Normal,
                                            arguments: Some(
                                                ApplyCallTerms {
                                                    terms: [
                                                        ApplyCallItem {
                                                            modifiers: ModifiersNode {
                                                                terms: [],
                                                            },
                                                            parameter: None,
                                                            body: Number(
                                                                NumberLiteralNode {
                                                                    value: "1",
                                                                    unit: None,
                                                                    span: 164..164,
                                                                },
                                                            ),
                                                            span: 164..164,
                                                        },
                                                    ],
                                                    span: 163..165,
                                                },
                                            ),
                                            trailing: None,
                                            span: 163..165,
                                        },
                                    ),
                                    monadic: false,
                                    caller: Internal(
                                        IdentifierNode {
                                            name: "b",
                                            span: 167..167,
                                        },
                                    ),
                                    arguments: Some(
                                        ApplyCallTerms {
                                            terms: [
                                                ApplyCallItem {
                                                    modifiers: ModifiersNode {
                                                        terms: [],
                                                    },
                                                    parameter: None,
                                                    body: Number(
                                                        NumberLiteralNode {
                                                            value: "1",
                                                            unit: None,
                                                            span: 169..169,
                                                        },
                                                    ),
                                                    span: 169..169,
                                                },
                                            ],
                                            span: 168..170,
                                        },
                                    ),
                                    trailing: None,
                                    span: 166..170,
                                },
                            ),
                            monadic: false,
                            caller: Normal,
                            arguments: Some(
                                ApplyCallTerms {
                                    terms: [
                                        ApplyCallItem {
                                            modifiers: ModifiersNode {
                                                terms: [],
                                            },
                                            parameter: None,
                                            body: Number(
                                                NumberLiteralNode {
                                                    value: "2",
                                                    unit: None,
                                                    span: 172..172,
                                                },
                                            ),
                                            span: 172..172,
                                        },
                                    ],
                                    span: 171..173,
                                },
                            ),
                            trailing: None,
                            span: 171..173,
                        },
                    ),
                    monadic: false,
                    caller: Internal(
                        IdentifierNode {
                            name: "b",
                            span: 175..175,
                        },
                    ),
                    arguments: Some(
                        ApplyCallTerms {
                            terms: [
                                ApplyCallItem {
                                    modifiers: ModifiersNode {
                                        terms: [],
                                    },
                                    parameter: None,
                                    body: Number(
                                        NumberLiteralNode {
                                            value: "1",
                                            unit: None,
                                            span: 177..177,
                                        },
                                    ),
                                    span: 177..177,
                                },
                            ],
                            span: 176..178,
                        },
                    ),
                    trailing: None,
                    span: 174..178,
                },
            ),
            monadic: false,
            caller: Normal,
            arguments: Some(
                ApplyCallTerms {
                    terms: [
                        ApplyCallItem {
                            modifiers: ModifiersNode {
                                terms: [],
                            },
                            parameter: None,
                            body: Number(
                                NumberLiteralNode {
                                    value: "2",
                                    unit: None,
                                    span: 180..180,
                                },
                            ),
                            span: 180..180,
                        },
                    ],
                    span: 179..181,
                },
            ),
            trailing: None,
            span: 179..181,
        },
    ),
    monadic: false,
    caller: Normal,
    arguments: Some(
        ApplyCallTerms {
            terms: [
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: None,
                    body: Number(
                        NumberLiteralNode {
                            value: "3",
                            unit: None,
                            span: 183..183,
                        },
                    ),
                    span: 183..183,
                },
            ],
            span: 182..184,
        },
    ),
    trailing: None,
    span: 182..184,
}
a∷b∷c
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "a",
                    span: 199..199,
                },
                IdentifierNode {
                    name: "b",
                    span: 202..202,
                },
                IdentifierNode {
                    name: "c",
                    span: 205..205,
                },
            ],
        },
    ),
    monadic: false,
    caller: Normal,
    arguments: Some(
        ApplyCallTerms {
            terms: [],
            span: 206..207,
        },
    ),
    trailing: None,
    span: 206..207,
}
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "a",
                    span: 212..212,
                },
                IdentifierNode {
                    name: "b",
                    span: 215..215,
                },
            ],
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "c",
            span: 217..217,
        },
    ),
    arguments: None,
    trailing: None,
    span: 216..217,
}
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "a",
                    span: 220..220,
                },
                IdentifierNode {
                    name: "b",
                    span: 223..223,
                },
            ],
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "c",
            span: 225..225,
        },
    ),
    arguments: Some(
        ApplyCallTerms {
            terms: [],
            span: 226..227,
        },
    ),
    trailing: None,
    span: 224..227,
}
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "a",
                    span: 232..232,
                },
            ],
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "b",
            span: 234..234,
        },
    ),
    arguments: None,
    trailing: None,
    span: 233..234,
}
c
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "a",
                    span: 240..240,
                },
            ],
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "b",
            span: 242..242,
        },
    ),
    arguments: None,
    trailing: None,
    span: 241..242,
}
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "c",
                    span: 245..245,
                },
            ],
        },
    ),
    monadic: false,
    caller: Normal,
    arguments: Some(
        ApplyCallTerms {
            terms: [],
            span: 246..247,
        },
    ),
    trailing: None,
    span: 246..247,
}
ClosureCallNode {
    base: LambdaCall(
        ClosureCallNode {
            base: Symbol(
                NamePathNode {
                    names: [
                        IdentifierNode {
                            name: "a",
                            span: 252..252,
                        },
                    ],
                },
            ),
            monadic: false,
            caller: Internal(
                IdentifierNode {
                    name: "b",
                    span: 254..254,
                },
            ),
            arguments: None,
            trailing: None,
            span: 253..254,
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "c",
            span: 256..256,
        },
    ),
    arguments: None,
    trailing: None,
    span: 255..256,
}
ClosureCallNode {
    base: LambdaCall(
        ClosureCallNode {
            base: Symbol(
                NamePathNode {
                    names: [
                        IdentifierNode {
                            name: "a",
                            span: 271..271,
                        },
                    ],
                },
            ),
            monadic: false,
            caller: Internal(
                IdentifierNode {
                    name: "b",
                    span: 273..273,
                },
            ),
            arguments: None,
            trailing: None,
            span: 272..273,
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "c",
            span: 275..275,
        },
    ),
    arguments: Some(
        ApplyCallTerms {
            terms: [],
            span: 276..277,
        },
    ),
    trailing: None,
    span: 274..277,
}
ClosureCallNode {
    base: LambdaCall(
        ClosureCallNode {
            base: Symbol(
                NamePathNode {
                    names: [
                        IdentifierNode {
                            name: "a",
                            span: 280..280,
                        },
                    ],
                },
            ),
            monadic: false,
            caller: Internal(
                IdentifierNode {
                    name: "b",
                    span: 282..282,
                },
            ),
            arguments: Some(
                ApplyCallTerms {
                    terms: [],
                    span: 283..284,
                },
            ),
            trailing: None,
            span: 281..284,
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "c",
            span: 286..286,
        },
    ),
    arguments: Some(
        ApplyCallTerms {
            terms: [],
            span: 287..288,
        },
    ),
    trailing: None,
    span: 285..288,
}
ClosureCallNode {
    base: LambdaCall(
        ClosureCallNode {
            base: LambdaCall(
                ClosureCallNode {
                    base: Symbol(
                        NamePathNode {
                            names: [
                                IdentifierNode {
                                    name: "a",
                                    span: 291..291,
                                },
                            ],
                        },
                    ),
                    monadic: false,
                    caller: Normal,
                    arguments: Some(
                        ApplyCallTerms {
                            terms: [],
                            span: 292..293,
                        },
                    ),
                    trailing: None,
                    span: 292..293,
                },
            ),
            monadic: false,
            caller: Internal(
                IdentifierNode {
                    name: "b",
                    span: 295..295,
                },
            ),
            arguments: Some(
                ApplyCallTerms {
                    terms: [],
                    span: 296..297,
                },
            ),
            trailing: None,
            span: 294..297,
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "c",
            span: 299..299,
        },
    ),
    arguments: Some(
        ApplyCallTerms {
            terms: [],
            span: 300..301,
        },
    ),
    trailing: None,
    span: 298..301,
}
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "NSLayoutConstraint",
                    span: 365..382,
                },
            ],
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "activate",
            span: 384..391,
        },
    ),
    arguments: Some(
        ApplyCallTerms {
            terms: [
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: None,
                    body: LambdaCall(
                        ClosureCallNode {
                            base: LambdaCall(
                                ClosureCallNode {
                                    base: Symbol(
                                        NamePathNode {
                                            names: [
                                                IdentifierNode {
                                                    name: "label",
                                                    span: 399..403,
                                                },
                                            ],
                                        },
                                    ),
                                    monadic: false,
                                    caller: Internal(
                                        IdentifierNode {
                                            name: "topAnchor",
                                            span: 405..413,
                                        },
                                    ),
                                    arguments: None,
                                    trailing: None,
                                    span: 404..413,
                                },
                            ),
                            monadic: false,
                            caller: Internal(
                                IdentifierNode {
                                    name: "constraint",
                                    span: 415..424,
                                },
                            ),
                            arguments: Some(
                                ApplyCallTerms {
                                    terms: [
                                        ApplyCallItem {
                                            modifiers: ModifiersNode {
                                                terms: [],
                                            },
                                            parameter: Some(
                                                IdentifierNode {
                                                    name: "equalTo",
                                                    span: 436..442,
                                                },
                                            ),
                                            body: LambdaCall(
                                                ClosureCallNode {
                                                    base: Symbol(
                                                        NamePathNode {
                                                            names: [
                                                                IdentifierNode {
                                                                    name: "button",
                                                                    span: 445..450,
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    monadic: false,
                                                    caller: Internal(
                                                        IdentifierNode {
                                                            name: "bottomAnchor",
                                                            span: 452..463,
                                                        },
                                                    ),
                                                    arguments: None,
                                                    trailing: None,
                                                    span: 451..463,
                                                },
                                            ),
                                            span: 436..463,
                                        },
                                        ApplyCallItem {
                                            modifiers: ModifiersNode {
                                                terms: [],
                                            },
                                            parameter: Some(
                                                IdentifierNode {
                                                    name: "constant",
                                                    span: 475..482,
                                                },
                                            ),
                                            body: Number(
                                                NumberLiteralNode {
                                                    value: "20",
                                                    unit: None,
                                                    span: 485..486,
                                                },
                                            ),
                                            span: 475..486,
                                        },
                                    ],
                                    span: 425..493,
                                },
                            ),
                            trailing: None,
                            span: 414..493,
                        },
                    ),
                    span: 399..493,
                },
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: None,
                    body: LambdaCall(
                        ClosureCallNode {
                            base: LambdaCall(
                                ClosureCallNode {
                                    base: Symbol(
                                        NamePathNode {
                                            names: [
                                                IdentifierNode {
                                                    name: "label",
                                                    span: 501..505,
                                                },
                                            ],
                                        },
                                    ),
                                    monadic: false,
                                    caller: Internal(
                                        IdentifierNode {
                                            name: "leadingAnchor",
                                            span: 507..519,
                                        },
                                    ),
                                    arguments: None,
                                    trailing: None,
                                    span: 506..519,
                                },
                            ),
                            monadic: false,
                            caller: Internal(
                                IdentifierNode {
                                    name: "constraint",
                                    span: 521..530,
                                },
                            ),
                            arguments: Some(
                                ApplyCallTerms {
                                    terms: [
                                        ApplyCallItem {
                                            modifiers: ModifiersNode {
                                                terms: [],
                                            },
                                            parameter: Some(
                                                IdentifierNode {
                                                    name: "equalTo",
                                                    span: 542..548,
                                                },
                                            ),
                                            body: LambdaCall(
                                                ClosureCallNode {
                                                    base: Symbol(
                                                        NamePathNode {
                                                            names: [
                                                                IdentifierNode {
                                                                    name: "button",
                                                                    span: 551..556,
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    monadic: false,
                                                    caller: Internal(
                                                        IdentifierNode {
                                                            name: "leadingAnchor",
                                                            span: 558..570,
                                                        },
                                                    ),
                                                    arguments: None,
                                                    trailing: None,
                                                    span: 557..570,
                                                },
                                            ),
                                            span: 542..570,
                                        },
                                    ],
                                    span: 531..577,
                                },
                            ),
                            trailing: None,
                            span: 520..577,
                        },
                    ),
                    span: 501..577,
                },
                ApplyCallItem {
                    modifiers: ModifiersNode {
                        terms: [],
                    },
                    parameter: None,
                    body: LambdaCall(
                        ClosureCallNode {
                            base: LambdaCall(
                                ClosureCallNode {
                                    base: Symbol(
                                        NamePathNode {
                                            names: [
                                                IdentifierNode {
                                                    name: "label",
                                                    span: 585..589,
                                                },
                                            ],
                                        },
                                    ),
                                    monadic: false,
                                    caller: Internal(
                                        IdentifierNode {
                                            name: "widthAnchor",
                                            span: 591..601,
                                        },
                                    ),
                                    arguments: None,
                                    trailing: None,
                                    span: 590..601,
                                },
                            ),
                            monadic: false,
                            caller: Internal(
                                IdentifierNode {
                                    name: "constraint",
                                    span: 603..612,
                                },
                            ),
                            arguments: Some(
                                ApplyCallTerms {
                                    terms: [
                                        ApplyCallItem {
                                            modifiers: ModifiersNode {
                                                terms: [],
                                            },
                                            parameter: Some(
                                                IdentifierNode {
                                                    name: "lessThanOrEqualTo",
                                                    span: 624..640,
                                                },
                                            ),
                                            body: LambdaCall(
                                                ClosureCallNode {
                                                    base: Symbol(
                                                        NamePathNode {
                                                            names: [
                                                                IdentifierNode {
                                                                    name: "view",
                                                                    span: 643..646,
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    monadic: false,
                                                    caller: Internal(
                                                        IdentifierNode {
                                                            name: "widthAnchor",
                                                            span: 648..658,
                                                        },
                                                    ),
                                                    arguments: None,
                                                    trailing: None,
                                                    span: 647..658,
                                                },
                                            ),
                                            span: 624..658,
                                        },
                                        ApplyCallItem {
                                            modifiers: ModifiersNode {
                                                terms: [],
                                            },
                                            parameter: Some(
                                                IdentifierNode {
                                                    name: "constant",
                                                    span: 670..677,
                                                },
                                            ),
                                            body: Prefix(
                                                PrefixNode {
                                                    operator: OperatorNode {
                                                        kind: Negative,
                                                        span: 680..680,
                                                    },
                                                    base: Number(
                                                        NumberLiteralNode {
                                                            value: "40",
                                                            unit: None,
                                                            span: 681..682,
                                                        },
                                                    ),
                                                    span: 680..682,
                                                },
                                            ),
                                            span: 670..682,
                                        },
                                    ],
                                    span: 613..689,
                                },
                            ),
                            trailing: None,
                            span: 602..689,
                        },
                    ),
                    span: 585..689,
                },
            ],
            span: 392..692,
        },
    ),
    trailing: None,
    span: 383..692,
}
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "label",
                    span: 695..699,
                },
            ],
        },
    ),
    monadic: false,
    caller: Internal(
        IdentifierNode {
            name: "layout",
            span: 701..706,
        },
    ),
    arguments: None,
    trailing: None,
    span: 700..706,
}
ClosureCallNode {
    base: Symbol(
        NamePathNode {
            names: [
                IdentifierNode {
                    name: "scope",
                    span: 855..859,
                },
            ],
        },
    ),
    monadic: false,
    caller: Normal,
    arguments: None,
    trailing: Some(
        FunctionBlock {
            terms: [],
            range: 861..906,
        },
    ),
    span: 861..906,
}
