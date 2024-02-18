import pytest
pytestmark = pytest.mark.lindera

from tantivy import SchemaBuilder, Index, Document, lindera


@pytest.mark.parametrize("mode", [
    lindera.LNormal(),
    lindera.LDecompose(),
])
def test_basic(mode):
    # breakpoint()
    sb = SchemaBuilder()
    sb.add_text_field("title", stored=True, tokenizer_name="lang_ja")
    schema = sb.build()
    index = Index(schema)
    index.register_lindera_tokenizer(
        "lang_ja",
        mode,
        lindera.LinderaDictionaryKind.IPADIC,
    )
    writer = index.writer(50_000_000)
    doc = Document()
    doc.add_text("title", "成田国際空港")
    writer.add_document(doc)
    writer.commit()
    index.reload()
