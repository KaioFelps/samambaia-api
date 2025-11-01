function decodeHtml(html: string) {
  const txt = document.createElement("textarea");
  txt.innerHTML = html;
  return txt.value;
}

export function equalHtml(a?: string, b?: string) {
  if (!a && !b) return true;
  if (!a || !b) return false;
  const decodedA = decodeHtml(a);
  const decodedB = decodeHtml(b);
  return decodedA === decodedB;
}
