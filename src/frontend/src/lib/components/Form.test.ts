import Form from "./Form.svelte";
import { render, screen } from "@testing-library/svelte";
import { expect, test, describe } from "vitest";
import { set } from "$lib/api.mock";

describe("Form", () => {
  test("should display form elements", async () => {
    render(Form, { set });

    const submitButton = screen.getByTestId("button");
    expect(submitButton.textContent).toBe("Send");

    submitButton.click();
    expect(set).toHaveBeenCalledWith("");
  });
});
