import { $NextraMetadata, Heading } from "nextra";
import { useMDXComponents as mdxComponents } from "nextra-theme-docs";
import { generateStaticParamsFor, importPage } from "nextra/pages";

export const generateStaticParams = generateStaticParamsFor("mdxPath");

type PageParams = {
  mdxPath?: string[];
};

type PageProps = {
  params: Promise<PageParams>;
};

type ContentProps = Omit<PageProps, "params"> & {
  params: PageParams;
};

type PageData = {
  default: React.ComponentType<ContentProps>;
  toc: Heading[];
  metadata: $NextraMetadata;
};

export const generateMetadata = async (props: PageProps) => {
  const { mdxPath } = await props.params;
  const { metadata } = (await importPage(mdxPath)) as PageData;
  const path = mdxPath ?? [];
  // If this isn't the root, add Bilinear Audio as a suffix.
  if (path.length > 0) {
    metadata.title = `${metadata.title} - Bilinear Audio`;
  }
  return metadata;
};

const Page = async (props: PageProps) => {
  const params = await props.params;
  const data = (await importPage(params.mdxPath)) as PageData;
  const { default: MDXContent, toc, metadata } = data;
  const Wrapper = mdxComponents().wrapper;

  return (
    <Wrapper toc={toc} metadata={metadata}>
      <MDXContent {...props} params={params} />
    </Wrapper>
  );
};

export default Page;
