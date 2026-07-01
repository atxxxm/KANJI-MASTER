import { motion } from "framer-motion";

interface Option {
  value: string;
  label: string;
}

interface Props {
  value: string;
  onChange: (v: string) => void;
  options: [Option, Option];
  className?: string;
}

export default function ModeSwitch({ value, onChange, options, className }: Props) {
  return (
    <div className={`mode-toggle${className ? ` ${className}` : ""}`}>
      <motion.div
        className="mode-pill"
        animate={{ x: value === options[0].value ? 0 : "100%" }}
        transition={{ type: "spring", stiffness: 500, damping: 38 }}
      />
      {options.map(opt => (
        <button
          key={opt.value}
          className={`mode-btn${value === opt.value ? " active" : ""}`}
          onClick={() => onChange(opt.value)}
        >
          {opt.label}
        </button>
      ))}
    </div>
  );
}
