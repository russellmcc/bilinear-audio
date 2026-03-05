import ContentCard from "./ContentCard";

const InfoCard = () => (
  <ContentCard>
    <h3>System requirements</h3>
    <ul>
      <li>macOS or Windows systems</li>
      <li>VST3</li>
    </ul>
    <h3>Getting help</h3>
    <ul>
      <li>
        Report issues and feature requests on{" "}
        <a href="https://github.com/russellmcc/bilinear-audio/discussions">
          GitHub
        </a>
      </li>
    </ul>
  </ContentCard>
);

export default InfoCard;
