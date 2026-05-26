import { useState, type CSSProperties } from "react";
import type { EnsemblePlusVoiceMode } from "./constants";

export type VoiceModeButtonProps = {
  mode: EnsemblePlusVoiceMode;
  active: boolean;
  onSelect: (mode: EnsemblePlusVoiceMode) => void;
};

const VoiceModeButton = ({ mode, active, onSelect }: VoiceModeButtonProps) => {
  const isHumanVoice = mode === "humanVoice";
  const fill = isHumanVoice ? "#d1ce43" : "#04a777";
  const shade = isHumanVoice ? "#7b791e" : "#025039";
  const border = isHumanVoice ? "#525114" : "#013223";
  const [pressed, setPressed] = useState(false);
  const [hovered, setHovered] = useState(false);
  const borderOffset = !pressed && hovered ? 3 : 4;
  const fillGroupStyle: CSSProperties = {
    position: "absolute",
    left: "0px",
    top: "0px",
    width: `${61 - borderOffset}px`,
    height: `${108 - borderOffset}px`,
    transform: pressed ? "translate(4px, 4px)" : "translate(0px, 0px)",
    transition: "height 200ms ease, transform 200ms ease, width 200ms ease",
  };

  return (
    <button
      type="button"
      aria-pressed={active}
      aria-label={isHumanVoice ? "Human voice mode" : "Strings mode"}
      onClick={() => {
        onSelect(mode);
      }}
      onPointerDown={() => {
        setPressed(true);
      }}
      onPointerEnter={() => {
        setHovered(true);
      }}
      onPointerUp={() => {
        setPressed(false);
      }}
      onPointerCancel={() => {
        setPressed(false);
      }}
      onPointerLeave={() => {
        setPressed(false);
        setHovered(false);
      }}
      onKeyDown={(event) => {
        if (event.key === " " || event.key === "Enter") {
          setPressed(true);
        }
      }}
      onKeyUp={() => {
        setPressed(false);
      }}
      onBlur={() => {
        setPressed(false);
      }}
      style={{
        position: "relative",
        width: "61px",
        height: "108px",
        borderRadius: "16px",
        overflow: "hidden",
        boxShadow: "2px 2px 2px rgba(0, 0, 0, 0.4)",
      }}
    >
      <span
        aria-hidden="true"
        style={{
          position: "absolute",
          left: "0px",
          top: "0px",
          width: "61px",
          height: "16px",
          borderTopLeftRadius: "16px",
          borderTopRightRadius: "16px",
          background: border,
        }}
      />
      <span
        aria-hidden="true"
        style={{
          position: "absolute",
          left: "0px",
          top: "16px",
          width: "61px",
          height: "92px",
          borderBottomLeftRadius: "16px",
          borderBottomRightRadius: "16px",
          background: border,
        }}
      ></span>
      <span aria-hidden="true" style={fillGroupStyle}>
        <span
          style={{
            position: "absolute",
            left: "0px",
            top: "0px",
            width: "100%",
            height: `${16 - Math.min(borderOffset, 2)}px`,
            borderTopLeftRadius: "16px",
            borderTopRightRadius: "16px",
            background: shade,
            transition: "height 200ms ease",
          }}
        />
        <span
          style={{
            position: "absolute",
            left: "0px",
            top: "16px",
            width: "100%",
            height: `${92 - borderOffset}px`,
            borderBottomLeftRadius: "16px",
            borderBottomRightRadius: "16px",
            background: fill,
            transition: "height 200ms ease",
          }}
        />
      </span>
      <span
        aria-hidden="true"
        style={{
          position: "absolute",
          left: "20px",
          top: "27px",
          width: "16px",
          height: "16px",
          borderRadius: "16px",
          background: active ? "#f83235" : "#1a0001",
          transition: "background 200ms ease",
        }}
      />
    </button>
  );
};

export default VoiceModeButton;
