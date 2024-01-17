describe('search bar',() => {

    let pages = [
        { origin: "/", input: "3NLXNij2mcewHRztr7Qdfmxk3a5FFBREqppcJiWiaChDErjdjZxg", tableHeading: 'Blocks' },
        { origin: "/summary", input: "3NLXNij2mcewHRztr7Qdfmxk3a5FFBREqppcJiWiaChDErjdjZxg", tableHeading: 'Blocks' },
        { origin: "/blocks", input: "3NLqPGGVtxXdsQg2orrp3SFFE3ToeMuqWRerSRWbmAKuSk2tphWy", tableHeading: 'Blocks' },
    ]

    pages.forEach(({origin, input, tableHeading}) => it(`works on ${origin} page`, () => {
        cy.visit(origin);
        cy.get("input#searchbar").type(input);
        cy.tableHasNRows(tableHeading, 1)
    }))
})