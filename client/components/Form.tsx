import { useContext, createContext } from "react";
type formObject = {
  id: string;
  name: string;
};
type FormProps<formObject> = {
  children: React.ReactNode;
  id: string;
  initialValue: formObject;
  onSubmit: (formObject: formObject) => null;
  onError: (formObject: formObject) => {};
};
// const FormContext = createContext({
//     value
// });
// export default function Form<formObject extends {}>({
//   children,
//   id,
//   onSubmit,
//   initialValue,
// }: FormProps<formObject>) {
//   <FormContext.Provider value={{ value: initialValue }}>
//     <form
//       id={id}
//       onSubmit={(e) => {
//         e.preventDefault();
//       }}
//     >
//       {children}
//     </form>
//   </FormContext.Provider>;
// }
// export function useForms<formObject>() {
//   let { value } = useContext(FormContext);
// }
