#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use u__IMPDEF_boolean::*;
use common::*;
pub fn MemHasTransactionalAccess<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memattrs: ProductTypef170cab34335b70c,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_20026: bool,
        gs_20025: bool,
        gs_20023: bool,
        ga_15444: ProductType183e6678e5239c85,
        gs_20021: bool,
        ga_15459: ProductType183e6678e5239c85,
        return_value: bool,
        ga_15453: ProductType183e6678e5239c85,
        ga_15456: ProductType183e6678e5239c85,
        ga_15447: ProductType183e6678e5239c85,
        gs_20020: bool,
        ga_15450: ProductType183e6678e5239c85,
        gs_20022: bool,
        gs_20027: bool,
        gs_20024: bool,
        memattrs: ProductTypef170cab34335b70c,
    }
    let fn_state = FunctionState {
        memattrs,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var memattrs.5:struct
        let s_0_0: u32 = fn_state.memattrs._5;
        // C s_0_1: const #1u : u32
        let s_0_1: u32 = 1;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b27 b1
        if s_0_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var memattrs.5:struct
        let s_1_0: u32 = fn_state.memattrs._5;
        // C s_1_1: const #2u : u32
        let s_1_1: u32 = 2;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // D s_1_3: write-var gs#20020 <= s_1_2
        fn_state.gs_20020 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#20020:u8
        let s_2_0: bool = fn_state.gs_20020;
        // N s_2_1: branch s_2_0 b26 b3
        if s_2_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#20021 <= s_3_0
        fn_state.gs_20021 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#20021:u8
        let s_4_0: bool = fn_state.gs_20021;
        // N s_4_1: branch s_4_0 b25 b5
        if s_4_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#20022 <= s_5_0
        fn_state.gs_20022 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#20022:u8
        let s_6_0: bool = fn_state.gs_20022;
        // N s_6_1: branch s_6_0 b24 b7
        if s_6_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#20023 <= s_7_0
        fn_state.gs_20023 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#20023:u8
        let s_8_0: bool = fn_state.gs_20023;
        // N s_8_1: branch s_8_0 b23 b9
        if s_8_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#20024 <= s_9_0
        fn_state.gs_20024 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#20024:u8
        let s_10_0: bool = fn_state.gs_20024;
        // N s_10_1: branch s_10_0 b22 b11
        if s_10_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#20025 <= s_11_0
        fn_state.gs_20025 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#20025:u8
        let s_12_0: bool = fn_state.gs_20025;
        // N s_12_1: branch s_12_0 b21 b13
        if s_12_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#20026 <= s_13_0
        fn_state.gs_20026 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var gs#20026:u8
        let s_14_0: bool = fn_state.gs_20026;
        // N s_14_1: branch s_14_0 b20 b15
        if s_14_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#20027 <= s_15_0
        fn_state.gs_20027 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var gs#20027:u8
        let s_16_0: bool = fn_state.gs_20027;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #"Memory Region does not support Transactional access" : str
        let s_17_0: &'static str = "Memory Region does not support Transactional access";
        // S s_17_1: call __IMPDEF_boolean(s_17_0)
        let s_17_1: bool = u__IMPDEF_boolean(state, tracer, s_17_0);
        // D s_17_2: write-var return_value <= s_17_1
        fn_state.return_value = s_17_1;
        // N s_17_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var return_value:u8
        let s_18_0: bool = fn_state.return_value;
        // N s_18_1: return s_18_0
        return s_18_0;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var return_value <= s_19_0
        fn_state.return_value = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var memattrs.4:struct
        let s_20_0: ProductType183e6678e5239c85 = fn_state.memattrs._4;
        // D s_20_1: write-var ga#15459 <= s_20_0
        fn_state.ga_15459 = s_20_0;
        // D s_20_2: read-var ga#15459.2:struct
        let s_20_2: bool = fn_state.ga_15459._2;
        // C s_20_3: const #0u : u8
        let s_20_3: bool = false;
        // D s_20_4: cmp-eq s_20_2 s_20_3
        let s_20_4: bool = ((s_20_2) == (s_20_3));
        // D s_20_5: write-var gs#20027 <= s_20_4
        fn_state.gs_20027 = s_20_4;
        // N s_20_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var memattrs.4:struct
        let s_21_0: ProductType183e6678e5239c85 = fn_state.memattrs._4;
        // D s_21_1: write-var ga#15456 <= s_21_0
        fn_state.ga_15456 = s_21_0;
        // D s_21_2: read-var ga#15456.0:struct
        let s_21_2: u8 = fn_state.ga_15456._0;
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 2u16);
        // C s_21_4: const #480u : u32
        let s_21_4: u32 = 480;
        // D s_21_5: read-reg s_21_4:u8
        let s_21_5: u8 = {
            let value = state.read_register::<u8>(s_21_4 as isize);
            tracer.read_register(s_21_4 as isize, value);
            value
        };
        // D s_21_6: cast zx s_21_5 -> bv
        let s_21_6: Bits = Bits::new(s_21_5 as u128, 2u16);
        // D s_21_7: cmp-eq s_21_3 s_21_6
        let s_21_7: bool = ((s_21_3) == (s_21_6));
        // D s_21_8: write-var gs#20026 <= s_21_7
        fn_state.gs_20026 = s_21_7;
        // N s_21_9: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var memattrs.4:struct
        let s_22_0: ProductType183e6678e5239c85 = fn_state.memattrs._4;
        // D s_22_1: write-var ga#15453 <= s_22_0
        fn_state.ga_15453 = s_22_0;
        // D s_22_2: read-var ga#15453.1:struct
        let s_22_2: u8 = fn_state.ga_15453._1;
        // D s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 2u16);
        // C s_22_4: const #512u : u32
        let s_22_4: u32 = 512;
        // D s_22_5: read-reg s_22_4:u8
        let s_22_5: u8 = {
            let value = state.read_register::<u8>(s_22_4 as isize);
            tracer.read_register(s_22_4 as isize, value);
            value
        };
        // D s_22_6: cast zx s_22_5 -> bv
        let s_22_6: Bits = Bits::new(s_22_5 as u128, 2u16);
        // D s_22_7: cmp-eq s_22_3 s_22_6
        let s_22_7: bool = ((s_22_3) == (s_22_6));
        // D s_22_8: write-var gs#20025 <= s_22_7
        fn_state.gs_20025 = s_22_7;
        // N s_22_9: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var memattrs.1:struct
        let s_23_0: ProductType183e6678e5239c85 = fn_state.memattrs._1;
        // D s_23_1: write-var ga#15450 <= s_23_0
        fn_state.ga_15450 = s_23_0;
        // D s_23_2: read-var ga#15450.2:struct
        let s_23_2: bool = fn_state.ga_15450._2;
        // C s_23_3: const #0u : u8
        let s_23_3: bool = false;
        // D s_23_4: cmp-eq s_23_2 s_23_3
        let s_23_4: bool = ((s_23_2) == (s_23_3));
        // D s_23_5: write-var gs#20024 <= s_23_4
        fn_state.gs_20024 = s_23_4;
        // N s_23_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var memattrs.1:struct
        let s_24_0: ProductType183e6678e5239c85 = fn_state.memattrs._1;
        // D s_24_1: write-var ga#15447 <= s_24_0
        fn_state.ga_15447 = s_24_0;
        // D s_24_2: read-var ga#15447.1:struct
        let s_24_2: u8 = fn_state.ga_15447._1;
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 2u16);
        // C s_24_4: const #512u : u32
        let s_24_4: u32 = 512;
        // D s_24_5: read-reg s_24_4:u8
        let s_24_5: u8 = {
            let value = state.read_register::<u8>(s_24_4 as isize);
            tracer.read_register(s_24_4 as isize, value);
            value
        };
        // D s_24_6: cast zx s_24_5 -> bv
        let s_24_6: Bits = Bits::new(s_24_5 as u128, 2u16);
        // D s_24_7: cmp-eq s_24_3 s_24_6
        let s_24_7: bool = ((s_24_3) == (s_24_6));
        // D s_24_8: write-var gs#20023 <= s_24_7
        fn_state.gs_20023 = s_24_7;
        // N s_24_9: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var memattrs.1:struct
        let s_25_0: ProductType183e6678e5239c85 = fn_state.memattrs._1;
        // D s_25_1: write-var ga#15444 <= s_25_0
        fn_state.ga_15444 = s_25_0;
        // D s_25_2: read-var ga#15444.0:struct
        let s_25_2: u8 = fn_state.ga_15444._0;
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 2u16);
        // C s_25_4: const #480u : u32
        let s_25_4: u32 = 480;
        // D s_25_5: read-reg s_25_4:u8
        let s_25_5: u8 = {
            let value = state.read_register::<u8>(s_25_4 as isize);
            tracer.read_register(s_25_4 as isize, value);
            value
        };
        // D s_25_6: cast zx s_25_5 -> bv
        let s_25_6: Bits = Bits::new(s_25_5 as u128, 2u16);
        // D s_25_7: cmp-eq s_25_3 s_25_6
        let s_25_7: bool = ((s_25_3) == (s_25_6));
        // D s_25_8: write-var gs#20022 <= s_25_7
        fn_state.gs_20022 = s_25_7;
        // N s_25_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var memattrs.2:struct
        let s_26_0: u32 = fn_state.memattrs._2;
        // C s_26_1: const #0u : u32
        let s_26_1: u32 = 0;
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // D s_26_3: write-var gs#20021 <= s_26_2
        fn_state.gs_20021 = s_26_2;
        // N s_26_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#20020 <= s_27_0
        fn_state.gs_20020 = s_27_0;
        // N s_27_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
