import { Knob, KnobProps } from "music-ui/draft-ui";
import { Decorator } from "@storybook/react-vite";
import { useState } from "react";

const GrabDecorator: Decorator<KnobProps> = (Story, context) => {
  const [grabbed, onGrabOrRelease] = useState(context.args.grabbed);

  return (
    <Story {...context} args={{ ...context.args, onGrabOrRelease, grabbed }} />
  );
};

const ValueDecorator: Decorator<KnobProps> = (Story, context) => {
  const [value, onValue] = useState(context.args.value);
  return <Story {...context} args={{ ...context.args, onValue, value }} />;
};

export default {
  component: Knob,
  decorators: [GrabDecorator, ValueDecorator],
  title: "Knob",
  tags: ["autodocs"],
  argTypes: {
    value: {
      table: {
        disable: true,
      },
    },
    grabbed: {
      table: {
        disable: true,
      },
    },
    valueFormatter: {
      table: {
        disable: true,
      },
    },
    onValue: {
      table: {
        disable: true,
      },
    },
    onGrabOrRelease: {
      table: {
        disable: true,
      },
    },
    defaultValue: {
      control: {
        type: "range",
        min: 0,
        max: 100,
        step: 1,
      },
    },
  },
};

export const Default = {
  args: {
    value: 50,
    label: "amount",
    grabbed: false,
    defaultValue: 50,
  },
};
