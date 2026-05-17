import { useState, type CSSProperties } from "react";
import type { Ju60ButtonMode } from "./constants";

const MODE_BUTTON_CONFIG = {
  I: {
    fill: "#f5e0b7",
    fillDark: "#dccaa5",
    shadow: "#93866e",
  },
  II: {
    fill: "#f4a889",
    fillDark: "#dc977b",
    shadow: "#936552",
  },
  III: {
    fill: "#f45b69",
    fillDark: "#dc525e",
    shadow: "#92373f",
  },
} satisfies Record<
  Ju60ButtonMode,
  {
    fill: string;
    fillDark: string;
    shadow: string;
  }
>;

type ModeButtonProps = {
  mode: Ju60ButtonMode;
  active: boolean;
  onSelect: (mode: Ju60ButtonMode) => void;
  onSelectByOffset: (mode: Ju60ButtonMode, offset: number) => void;
};

const ModeButton = ({
  mode,
  active,
  onSelect,
  onSelectByOffset,
}: ModeButtonProps) => {
  const config = MODE_BUTTON_CONFIG[mode];
  const [hovered, setHovered] = useState(false);
  const bevelWidth = !active && hovered ? "3.75px" : "5px";

  const innerStyle: CSSProperties = active
    ? {
        position: "absolute",
        top: bevelWidth,
        right: "0px",
        bottom: "0px",
        left: bevelWidth,
        borderRadius: "inherit",
        background: `linear-gradient(134.61489478758605deg, ${config.fill} 7.0881%, ${config.fillDark} 90.779%)`,
        transition: "inset 200ms ease",
      }
    : {
        position: "absolute",
        top: "0px",
        right: bevelWidth,
        bottom: bevelWidth,
        left: "0px",
        borderRadius: "inherit",
        background: `linear-gradient(134.61489478758605deg, ${config.fill} 7.0881%, ${config.fillDark} 90.779%)`,
        transition: "inset 200ms ease",
      };

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        width: "80px",
      }}
    >
      <span
        style={{
          fontSize: "36px",
          lineHeight: "36px",
          color: "var(--text-color)",
        }}
      >
        {mode}
      </span>
      <span
        style={{
          display: "flex",
          justifyContent: "center",
          alignItems: "flex-start",
          width: "80px",
          height: "68px",
        }}
      >
        <span
          aria-hidden="true"
          style={{
            width: "12px",
            height: "12px",
            marginTop: "15px",
            borderRadius: "50%",
            background: "#c52233",
            opacity: active ? 1 : 0,
            transform: active ? "scale(1)" : "scale(0.65)",
            transition: active
              ? "opacity 300ms ease-in, transform 300ms ease-in"
              : "opacity 200ms ease-out, transform 200ms ease-out",
          }}
          data-active={active ? "true" : "false"}
        />
      </span>
      <span
        style={{
          position: "relative",
          width: "80px",
          height: "80px",
        }}
      >
        <div
          id={`ju-60-mode-${mode}`}
          role="radio"
          aria-checked={active}
          tabIndex={active ? 0 : -1}
          style={{
            position: "absolute",
            inset: "0px",
            zIndex: 1,
            width: "80px",
            height: "80px",
            margin: "0px",
            opacity: 0,
            cursor: "pointer",
          }}
          onPointerDown={(event) => {
            event.currentTarget.focus();
          }}
          onPointerEnter={() => {
            setHovered(true);
          }}
          onPointerLeave={() => {
            setHovered(false);
          }}
          onClick={(event) => {
            event.currentTarget.focus();
            onSelect(mode);
          }}
          onKeyDown={(event) => {
            if (event.key === "ArrowDown" || event.key === "ArrowRight") {
              event.preventDefault();
              onSelectByOffset(mode, 1);
            } else if (event.key === "ArrowUp" || event.key === "ArrowLeft") {
              event.preventDefault();
              onSelectByOffset(mode, -1);
            } else if (event.key === " " || event.key === "Enter") {
              event.preventDefault();
              onSelect(mode);
            }
          }}
          aria-label={`Synthesizer chorus mode ${mode}`}
        />
        <span
          aria-hidden="true"
          style={{
            display: "block",
            position: "relative",
            width: "80px",
            height: "80px",
            borderRadius: "10px",
            background: config.shadow,
            boxSizing: "border-box",
            overflow: "hidden",
          }}
          data-active={active ? "true" : "false"}
        >
          <span style={innerStyle} />
        </span>
      </span>
    </div>
  );
};

export default ModeButton;
