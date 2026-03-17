/**
 * Tests for the ContractDeployment component.
 *
 * Verifies form rendering, deploy button disabled state, and endowment
 * input interaction with a mocked useBlockchain hook.
 */
import React from "react";
import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import ContractDeployment from "../ContractDeployment";

// Mock useBlockchain
vi.mock("../../../contexts/BlockchainContext", () => ({
  useBlockchain: () => ({
    isDeployingContract: false,
    deployContract: vi.fn().mockResolvedValue({
      success: true,
      data: "0xmockaddress",
      txHash: "0xmocktx",
    }),
  }),
}));

describe("ContractDeployment", () => {
  it("renders deployment form", () => {
    render(<ContractDeployment onContractTypeChange={vi.fn()} />);
    expect(screen.getByText("Contract File (.wasm)")).toBeTruthy();
    expect(screen.getByText("Endowment (WND)")).toBeTruthy();
  });

  it("has a deploy button that starts disabled", () => {
    render(<ContractDeployment onContractTypeChange={vi.fn()} />);
    const buttons = screen.getAllByRole("button");
    const deployButton = buttons.find(
      (b) => b.textContent?.includes("Deploy Contract")
    );
    expect(deployButton).toBeTruthy();
    expect(deployButton).toBeDisabled();
  });

  it("allows changing endowment value", () => {
    render(<ContractDeployment onContractTypeChange={vi.fn()} />);
    const input = screen.getByDisplayValue("1.0");
    fireEvent.change(input, { target: { value: "2.5" } });
    expect(input).toHaveValue(2.5);
  });
});
