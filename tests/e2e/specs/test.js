// https://docs.cypress.io/api/introduction/api.html

describe("My First Test", () => {
  it("Visits the app root url", () => {
    cy.visit("/");
    cy.contains("button", "Run");
    cy.contains("button", "Animate");
    cy.contains("button", "Stop Animation");
    cy.get(".link[href=/settings]").click();
    cy.contains("h1", "Settings");
  });
});
