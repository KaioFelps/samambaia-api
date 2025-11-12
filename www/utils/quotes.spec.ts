import { decode } from "html-entities";
import { contentsAreEquivalent, decodeQuotes, encodeQuotes } from "./quotes";

describe("Articles Double Quotes Encoding", () => {
  const articleContent = `<p>Some "double quoted phrase". Also encodes 'singles'</p>`;
  const goodToGoAsJSON = encodeQuotes(articleContent);
  const json = JSON.stringify(goodToGoAsJSON);

  it(
    "should always encode a double quote twice so that it " +
      "doesn't break json after browser's HTMl decode",
    () => {
      expect(goodToGoAsJSON).toEqual(
        "<p>Some &amp;quot;double quoted phrase&amp;quot;. Also encodes &amp;apos;singles&amp;apos;</p>",
      );
    },
  );

  it(
    "should allow the double quote to be correctly present " +
      "after being deserialized as JSON and decoded as HTML component",
    () => {
      // this is the process that *actually happens* when this content is shiped from
      // the server through inertia page props — which are a stringified json placed
      // in a html attribute, thus html decoded.
      const initialDecode = decode(json); // as a stringified prop from #app.props
      const deserialized = JSON.parse(initialDecode);
      const finalDecode = decode(deserialized); // after injected as inner html

      expect(finalDecode).toEqual(articleContent);
    },
  );

  it("should keep already html-encoded quotes consistent", () => {
    const input = '<img src="" alt="This is a description with &quot;quotes&quot; already." />';
    const encodedInput = encodeQuotes(input);
    const jsonInput = JSON.stringify(encodedInput);

    expect(encodedInput).toEqual(
      "<img src=&amp;quot;&amp;quot; alt=&amp;quot;This is a description with " +
        "&amp;amp;quot;quotes&amp;amp;quot; already.&amp;quot; />",
    );

    expect(decodeQuotes(JSON.parse(jsonInput))).toEqual(input);
  });

  it("should be able to compare encoded and non-encoded versions of the same content", () => {
    expect(contentsAreEquivalent(articleContent, articleContent)).toBeTruthy();
    expect(contentsAreEquivalent(goodToGoAsJSON, goodToGoAsJSON)).toBeTruthy();
    expect(contentsAreEquivalent(articleContent, goodToGoAsJSON)).toBeTruthy();
    expect(contentsAreEquivalent(articleContent, decode(goodToGoAsJSON))).toBeTruthy();
    expect(contentsAreEquivalent(goodToGoAsJSON, decode(goodToGoAsJSON))).toBeTruthy();
  });

  it("should not declare distinct contents equivalent", () => {
    expect(contentsAreEquivalent("<p>Foo</p>", "<p>Foo </p>")).toBeFalsy();
    expect(contentsAreEquivalent("<p>Foo&apos;</p>", "<p>Foo&quot;</p>")).toBeFalsy();
    expect(contentsAreEquivalent("<p>Foo&amp;apos;</p>", "<p>Foo&amp;quot;</p>")).toBeFalsy();
    expect(contentsAreEquivalent("a", "b")).toBeFalsy();
  });
});

// describe("Articles Double Quotes Decoding", () => {
//   it("should correctly decode multiple encoded double quotes", () => {
//     const encodedContent =
//       "<p style=&amp;quot;text-align: center;&amp;quot;><img alt=&amp;quot;Um " +
//       "grande milho em pixel-arte. Na frente desse, est&aacute; escrito &amp;quot;cr&ocirc;nicas " +
//       "do milho&quot;.&amp;quot; src=&amp;quot;https://i.imgur.com/xu4OtAq.png&amp;quot;></p>";

//     const expectedDecodedContentAfterFirstDecode =
//       "<p style=&quot;text-align: center;&quot;>" +
//       "<img alt=&quot;Um grande milho em pixel-arte. Na frente desse, está escrito &quot;crônicas " +
//       "do milho&quot;. &quot; src";
//   });
// });
