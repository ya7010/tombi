import {
  RequestType,
  type TextDocumentIdentifier,
} from "vscode-languageclient";

export const getTomlVersion = new RequestType<
  GetTomlVersionParams,
  string,
  void
>("tombi/getTomlVersion");

export type GetTomlVersionParams = TextDocumentIdentifier;
