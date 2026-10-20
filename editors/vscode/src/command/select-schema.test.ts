import { beforeEach, describe, expect, it, vi } from "vitest";
import type * as node from "vscode-languageclient/node";

const mocks = vi.hoisted(() => ({
  text: "",
  showWarningMessage: vi.fn(),
  showInformationMessage: vi.fn(),
  showErrorMessage: vi.fn(),
  showQuickPick: vi.fn(),
  sendRequest: vi.fn(),
  sendNotification: vi.fn(),
}));

vi.mock("vscode", () => ({
  window: {
    get activeTextEditor() {
      return {
        document: {
          languageId: "toml",
          uri: { scheme: "file", fsPath: "/project/test.toml" },
          get lineCount() {
            return mocks.text.split("\n").length;
          },
          lineAt(line: number) {
            return { text: mocks.text.split("\n")[line] };
          },
        },
      };
    },
    showWarningMessage: mocks.showWarningMessage,
    showInformationMessage: mocks.showInformationMessage,
    showErrorMessage: mocks.showErrorMessage,
    showQuickPick: mocks.showQuickPick,
  },
}));
vi.mock("@/extension", () => ({ TOMBI_DEV_VERSION: "0.0.0-dev" }));
vi.mock("@/logging", () => ({
  log: { info: vi.fn(), warn: vi.fn(), error: vi.fn() },
}));
vi.mock("@/lsp/client", () => ({ listSchemas: {} }));

import { selectSchema } from "./select-schema";

const client = {
  sendRequest: mocks.sendRequest,
  sendNotification: mocks.sendNotification,
} as unknown as node.LanguageClient;

const schema = { title: "Selected", uri: "tombi://selected" };
describe("selectSchema", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.text = "answer = 42\n";
    mocks.sendRequest.mockResolvedValue({ schemas: [schema] });
    mocks.showQuickPick.mockResolvedValue({ label: schema.title, schema });
  });

  it("rejects a header schema directive before listing schemas", async () => {
    mocks.text = "\n  # note\n  #:schema ./schema.json\nanswer = 42\n";

    await selectSchema(client);

    expect(mocks.showErrorMessage).toHaveBeenCalledWith(
      "The schema cannot be changed for a file with `#:schema` directive.",
    );
    expect(mocks.sendRequest).not.toHaveBeenCalled();
    expect(mocks.showQuickPick).not.toHaveBeenCalled();
    expect(mocks.sendNotification).not.toHaveBeenCalled();
  });

  it("allows selection when the directive is below a TOML item", async () => {
    mocks.text = "answer = 42\n#:schema ./schema.json\n";

    await selectSchema(client);

    expect(mocks.sendNotification).toHaveBeenCalledWith(
      "tombi/associateSchema",
      expect.objectContaining({ uri: schema.uri, force: true }),
    );
    expect(mocks.showErrorMessage).not.toHaveBeenCalled();
  });

  it("does not treat a tab after #:schema as a schema directive", async () => {
    mocks.text = "#:schema\t./schema.json\nanswer = 42\n";

    await selectSchema(client);

    expect(mocks.sendNotification).toHaveBeenCalledOnce();
    expect(mocks.showErrorMessage).not.toHaveBeenCalled();
  });
});
