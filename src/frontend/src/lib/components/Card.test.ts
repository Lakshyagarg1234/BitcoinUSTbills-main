import Card from "./Card.svelte";
import { render } from "@testing-library/svelte";
import { describe, expect, test } from "vitest";

describe("Card", () => {
  test("should display title", () => {
    const { getByTestId } = render(Card, {
      principal: "Principal",
      string: "String",
    });

    expect(getByTestId("principal").textContent).toBe("Principal");
  });

  test("should display string", () => {
    const { getByTestId } = render(Card, {
      principal: "Principal",
      string: "String",
    });

    expect(getByTestId("string").textContent).toBe("String");
  });
});
