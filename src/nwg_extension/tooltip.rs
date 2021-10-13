use native_windows_gui as nwg;


pub trait OneArgRegister<'a> {
    fn one_arg_register(self, x: (&nwg::TextInput, &'a str)) -> Self;
    fn parent(self, w: &nwg::Window) -> Self;
}

impl <'a> OneArgRegister<'a> for nwg::TooltipBuilder<'a> {
    fn one_arg_register(self, x: (&nwg::TextInput, &'a str)) -> Self {
        self.register(x.0, x.1)
    }

    fn parent(self, _: &nwg::Window) -> Self {
        // TooltipBuilder doesn't have a parent method, but native-windows-derive
        // looks for it, so we need a dummy one
         self
    }
}
