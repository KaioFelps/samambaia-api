import clsx from "clsx";
import { memo } from "react";
import PrimitiveSelect, { defaultTheme } from "react-select";

import { Alert } from "@/components/alert";

export type SelectOption = { value: string; label: string };
export type SelectOptions = Array<SelectOption>;

type MultiSelectProps = {
  options: SelectOptions;
  defaultOptions?: SelectOptions;
  setValues: (_value: SelectOptions) => void;
};

const theme = defaultTheme;
theme.colors = {
  ...theme.colors,
  primary: "var(--color-purple-500)",
  primary75: "color-mix(in oklab, var(--color-purple-500) 75%, transparent)",
  primary50: "color-mix(in oklab, var(--color-purple-500) 50%, transparent)",
  primary25: "color-mix(in oklab, var(--color-purple-500) 25%, transparent)",
};

const MultiSelect = memo(({ options, defaultOptions, setValues }: MultiSelectProps) => {
  return (
    <PrimitiveSelect
      options={options}
      defaultValue={defaultOptions}
      isMulti
      isClearable
      isSearchable
      placeholder="Escolha uma tag"
      onChange={(value) => setValues(value as SelectOptions)}
      noOptionsMessage={() => (
        <Alert
          admin
          message="Não há tags registradas."
          type="warning"
          className="border-hidden rounded-lg"
        />
      )}
      theme={theme}
      styles={{
        control: (base, _state) => ({
          ...base,
          borderRadius: "calc(var(--spacing)*2)",
          fontSize: "var(--text-sm)",
          outline: "none",
          paddingBlock: 0,
          minHeight: "unset",
        }),
        valueContainer: (base, _state) => ({
          ...base,
          paddingLeft: "var(--spacing)",
        }),
        option: (base, state) => ({
          ...base,
          backgroundColor: clsx(
            state.isDisabled && "var(--color-gray-200)",
            state.isFocused && "color-mix(in oklab, var(--color-purple-500) 20%, transparent)",
          ),
        }),
        multiValue: (base, _state) => ({
          ...base,
          borderRadius: "calc(var(--spacing)*2)",
          backgroundColor: "color-mix(in oklab, var(--color-purple-500) 25%, transparent)",
          overflow: "hidden",
        }),
        multiValueRemove: (base, _) => ({
          ...base,
          paddingBlock: 0,
          minHeight: "unset",
        }),
        multiValueLabel: (base, _state) => ({
          ...base,
          fontSize: "var(--text-sm)",
          lineHeight: "var(--leading-tight)",
        }),
        dropdownIndicator: (base, _) => ({
          ...base,
          paddingBlock: 0,
        }),
        clearIndicator: (base, _) => ({
          ...base,
          paddingBlock: 0,
        }),
      }}
    />
  );
});

export default MultiSelect;
