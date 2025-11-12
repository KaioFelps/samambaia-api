import { decode } from "html-entities";

const targets = Object.freeze([
  ["&quot;", "amp;quot"],
  ["&apos;", "amp;apos"],
  ["'", "apos"],
  ['"', "quot"],
]);

export function encodeQuotes(content: string) {
  let decodedContent = content;

  for (const [decoded, encoded] of targets) {
    decodedContent = decodedContent.replaceAll(decoded, `&amp;${encoded};`);
  }

  return decodedContent;
}

export function decodeQuotes(content: string) {
  return content
    .replaceAll("&amp;", "&")
    .replaceAll("&quot;", '"')
    .replaceAll("&apos;", "'")
    .replaceAll("&amp;", "&");
}

export function contentsAreEquivalent(a?: string, b?: string) {
  if (!a && !b) return true;
  if (!a || !b) return false;

  if (a.includes("&amp;quot;")) a = decodeQuotes(a);
  if (b.includes("&amp;quot;")) b = decodeQuotes(b);

  const decodedA = decode(a);
  const decodedB = decode(b);
  return decodedA === decodedB;
}
