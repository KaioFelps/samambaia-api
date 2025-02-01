import { Alert } from "@/components/alert";

export default function Foo() {
  return (
    <main>
      <Alert
        type="success"
        message="baz baz baz"
      />
      <Alert
        type="warning"
        message="baz baz baz"
      />
      <Alert
        type="error"
        message="baz baz baz"
      />
    </main>
  );
}
