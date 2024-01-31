fn foo() {
    let x = Ok(Ledger {
        transactions: [Transaction {
            date: Date {
                year: 2023,
                month: 1,
                day: 1,
                source_span: None,
            },
            description: "\"Mcdonald's\"",
            postings: [
                Posting {
                    account: Account {
                        path: ["expenses", "fast_food"],
                        source_span: Some(SourceSpan {
                            start_byte: 28,
                            end_byte: 46,
                            start_row: 1,
                            start_col: 4,
                            end_row: 1,
                            end_col: 22,
                        }),
                    },
                    amount: Amount {
                        cents: 1091,
                        source_span: Some(SourceSpan {
                            start_byte: 55,
                            end_byte: 60,
                            start_row: 1,
                            start_col: 31,
                            end_row: 1,
                            end_col: 36,
                        }),
                    },
                    source_span: Some(SourceSpan {
                        start_byte: 28,
                        end_byte: 60,
                        start_row: 1,
                        start_col: 4,
                        end_row: 1,
                        end_col: 36,
                    }),
                },
                Posting {
                    account: Account {
                        path: ["liabilities", "my_credit_card"],
                        source_span: Some(SourceSpan {
                            start_byte: 65,
                            end_byte: 91,
                            start_row: 2,
                            start_col: 4,
                            end_row: 2,
                            end_col: 30,
                        }),
                    },
                    amount: Amount {
                        cents: -1091,
                        source_span: Some(SourceSpan {
                            start_byte: 92,
                            end_byte: 98,
                            start_row: 2,
                            start_col: 31,
                            end_row: 2,
                            end_col: 37,
                        }),
                    },
                    source_span: Some(SourceSpan {
                        start_byte: 65,
                        end_byte: 98,
                        start_row: 2,
                        start_col: 4,
                        end_row: 2,
                        end_col: 37,
                    }),
                },
            ],
            source_span: Some(SourceSpan {
                start_byte: 0,
                end_byte: 98,
                start_row: 0,
                start_col: 0,
                end_row: 2,
                end_col: 37,
            }),
        }],
        source_span: Some(SourceSpan {
            start_byte: 0,
            end_byte: 98,
            start_row: 0,
            start_col: 0,
            end_row: 2,
            end_col: 37,
        }),
    });
}
