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
use getNumWatchpoints::*;
use u_get_CTR_EL0_Type_DminLine::*;
use getNumBreakpoints::*;
use getNumCtxBreakpoints::*;
use common::*;
pub fn ImpDefInt<T: Tracer>(state: &mut State, tracer: &T, x: &'static str) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        return_value: i128,
        x: &'static str,
    }
    let fn_state = FunctionState {
        x,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_0_0: read-var x:str
        let s_0_0: &'static str = fn_state.x;
        // C s_0_1: const #"Number of breakpoints" : str
        let s_0_1: &'static str = "Number of breakpoints";
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b39 b1
        if s_0_2 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_1_0: read-var x:str
        let s_1_0: &'static str = fn_state.x;
        // C s_1_1: const #"Number of watchpoints" : str
        let s_1_1: &'static str = "Number of watchpoints";
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b38 b2
        if s_1_2 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_2_0: read-var x:str
        let s_2_0: &'static str = fn_state.x;
        // C s_2_1: const #"Number of event counters" : str
        let s_2_1: &'static str = "Number of event counters";
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b37 b3
        if s_2_2 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_3_0: read-var x:str
        let s_3_0: &'static str = fn_state.x;
        // C s_3_1: const #"Number of context-aware breakpoints" : str
        let s_3_1: &'static str = "Number of context-aware breakpoints";
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b36 b4
        if s_3_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_4_0: read-var x:str
        let s_4_0: &'static str = fn_state.x;
        // C s_4_1: const #"Max implemented VL" : str
        let s_4_1: &'static str = "Max implemented VL";
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // N s_4_3: branch s_4_2 b35 b5
        if s_4_2 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_5_0: read-var x:str
        let s_5_0: &'static str = fn_state.x;
        // C s_5_1: const #"Maximum bytes used in a single block of a copy" : str
        let s_5_1: &'static str = "Maximum bytes used in a single block of a copy";
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // N s_5_3: branch s_5_2 b34 b6
        if s_5_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_6_0: read-var x:str
        let s_6_0: &'static str = fn_state.x;
        // C s_6_1: const #"Number of BRB records" : str
        let s_6_1: &'static str = "Number of BRB records";
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // N s_6_3: branch s_6_2 b33 b7
        if s_6_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_7_0: read-var x:str
        let s_7_0: &'static str = fn_state.x;
        // C s_7_1: const #"SPE Events packet payload size" : str
        let s_7_1: &'static str = "SPE Events packet payload size";
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b32 b8
        if s_7_2 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_8_0: read-var x:str
        let s_8_0: &'static str = fn_state.x;
        // C s_8_1: const #"SPE Data Source packet payload size" : str
        let s_8_1: &'static str = "SPE Data Source packet payload size";
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b31 b9
        if s_8_2 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_9_0: read-var x:str
        let s_9_0: &'static str = fn_state.x;
        // C s_9_1: const #"Reserved Intermediate Physical Address size value" : str
        let s_9_1: &'static str = "Reserved Intermediate Physical Address size value";
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // N s_9_3: branch s_9_2 b30 b10
        if s_9_2 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_10_0: read-var x:str
        let s_10_0: &'static str = fn_state.x;
        // C s_10_1: const #"Maximum Physical Address Size" : str
        let s_10_1: &'static str = "Maximum Physical Address Size";
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // N s_10_3: branch s_10_2 b29 b11
        if s_10_2 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_11_0: read-var x:str
        let s_11_0: &'static str = fn_state.x;
        // C s_11_1: const #"Block BBM support level" : str
        let s_11_1: &'static str = "Block BBM support level";
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // N s_11_3: branch s_11_2 b28 b12
        if s_11_2 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_12_0: read-var x:str
        let s_12_0: &'static str = fn_state.x;
        // C s_12_1: const #"SVE Extended BFloat16 instructions are implemented" : str
        let s_12_1: &'static str = "SVE Extended BFloat16 instructions are implemented";
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // N s_12_3: branch s_12_2 b27 b13
        if s_12_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_13_0: read-var x:str
        let s_13_0: &'static str = fn_state.x;
        // C s_13_1: const #"Max implemented SVL" : str
        let s_13_1: &'static str = "Max implemented SVL";
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b26 b14
        if s_13_2 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_14_0: read-var x:str
        let s_14_0: &'static str = fn_state.x;
        // C s_14_1: const #"Data Cache Invalidate Watchpoint Size" : str
        let s_14_1: &'static str = "Data Cache Invalidate Watchpoint Size";
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // N s_14_3: branch s_14_2 b25 b15
        if s_14_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_15_0: read-var x:str
        let s_15_0: &'static str = fn_state.x;
        // C s_15_1: const #"Instruction Cache Invalidate by VA Size" : str
        let s_15_1: &'static str = "Instruction Cache Invalidate by VA Size";
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // N s_15_3: branch s_15_2 b24 b16
        if s_15_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_16_0: read-var x:str
        let s_16_0: &'static str = fn_state.x;
        // C s_16_1: const #"Linesize for DC SW instructions" : str
        let s_16_1: &'static str = "Linesize for DC SW instructions";
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // N s_16_3: branch s_16_2 b23 b17
        if s_16_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_17_0: read-var x:str
        let s_17_0: &'static str = fn_state.x;
        // C s_17_1: const #"Associativity for DC SW instructions" : str
        let s_17_1: &'static str = "Associativity for DC SW instructions";
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // N s_17_3: branch s_17_2 b22 b18
        if s_17_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_18_0: read-var x:str
        let s_18_0: &'static str = fn_state.x;
        // C s_18_1: const #"Numsets for DC SW instructions" : str
        let s_18_1: &'static str = "Numsets for DC SW instructions";
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // N s_18_3: branch s_18_2 b21 b19
        if s_18_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // N s_19_1: assert s_19_0
        let s_19_1: () = assert!(s_19_0);
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_20_0: read-var return_value:i
        let s_20_0: i128 = fn_state.return_value;
        // N s_20_1: return s_20_0
        return s_20_0;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_21_0: const #2s : i
        let s_21_0: i128 = 2;
        // D s_21_1: write-var return_value <= s_21_0
        fn_state.return_value = s_21_0;
        // N s_21_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_22_0: const #2s : i
        let s_22_0: i128 = 2;
        // D s_22_1: write-var return_value <= s_22_0
        fn_state.return_value = s_22_0;
        // N s_22_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_23_0: const #19056u : u32
        let s_23_0: u32 = 19056;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_CTR_EL0_Type_DminLine(s_23_1)
        let s_23_2: u8 = u_get_CTR_EL0_Type_DminLine(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 4u16);
        // D s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (s_23_3.value() as i128);
        // D s_23_5: cast reint s_23_4 -> i64
        let s_23_5: i64 = (s_23_4 as i64);
        // D s_23_6: cast zx s_23_5 -> i
        let s_23_6: i128 = (i128::try_from(s_23_5).unwrap());
        // D s_23_7: pow2 s_23_6
        let s_23_7: i128 = (s_23_6).pow(2);
        // D s_23_8: cast reint s_23_7 -> i64
        let s_23_8: i64 = (s_23_7 as i64);
        // C s_23_9: const #4s : i
        let s_23_9: i128 = 4;
        // D s_23_10: cast zx s_23_8 -> i
        let s_23_10: i128 = (i128::try_from(s_23_8).unwrap());
        // D s_23_11: mul s_23_9 s_23_10
        let s_23_11: i128 = ((s_23_9) * (s_23_10));
        // D s_23_12: cast reint s_23_11 -> i64
        let s_23_12: i64 = (s_23_11 as i64);
        // D s_23_13: cast zx s_23_12 -> i
        let s_23_13: i128 = (i128::try_from(s_23_12).unwrap());
        // D s_23_14: write-var return_value <= s_23_13
        fn_state.return_value = s_23_13;
        // N s_23_15: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_24_0: const #19056u : u32
        let s_24_0: u32 = 19056;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_CTR_EL0_Type_DminLine(s_24_1)
        let s_24_2: u8 = u_get_CTR_EL0_Type_DminLine(state, tracer, s_24_1);
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 4u16);
        // D s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (s_24_3.value() as i128);
        // D s_24_5: cast reint s_24_4 -> i64
        let s_24_5: i64 = (s_24_4 as i64);
        // D s_24_6: cast zx s_24_5 -> i
        let s_24_6: i128 = (i128::try_from(s_24_5).unwrap());
        // D s_24_7: pow2 s_24_6
        let s_24_7: i128 = (s_24_6).pow(2);
        // D s_24_8: cast reint s_24_7 -> i64
        let s_24_8: i64 = (s_24_7 as i64);
        // C s_24_9: const #4s : i
        let s_24_9: i128 = 4;
        // D s_24_10: cast zx s_24_8 -> i
        let s_24_10: i128 = (i128::try_from(s_24_8).unwrap());
        // D s_24_11: mul s_24_9 s_24_10
        let s_24_11: i128 = ((s_24_9) * (s_24_10));
        // D s_24_12: cast reint s_24_11 -> i64
        let s_24_12: i64 = (s_24_11 as i64);
        // D s_24_13: cast zx s_24_12 -> i
        let s_24_13: i128 = (i128::try_from(s_24_12).unwrap());
        // D s_24_14: write-var return_value <= s_24_13
        fn_state.return_value = s_24_13;
        // N s_24_15: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_25_0: const #19056u : u32
        let s_25_0: u32 = 19056;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_CTR_EL0_Type_DminLine(s_25_1)
        let s_25_2: u8 = u_get_CTR_EL0_Type_DminLine(state, tracer, s_25_1);
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 4u16);
        // D s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (s_25_3.value() as i128);
        // D s_25_5: cast reint s_25_4 -> i64
        let s_25_5: i64 = (s_25_4 as i64);
        // D s_25_6: cast zx s_25_5 -> i
        let s_25_6: i128 = (i128::try_from(s_25_5).unwrap());
        // D s_25_7: pow2 s_25_6
        let s_25_7: i128 = (s_25_6).pow(2);
        // D s_25_8: cast reint s_25_7 -> i64
        let s_25_8: i64 = (s_25_7 as i64);
        // C s_25_9: const #4s : i
        let s_25_9: i128 = 4;
        // D s_25_10: cast zx s_25_8 -> i
        let s_25_10: i128 = (i128::try_from(s_25_8).unwrap());
        // D s_25_11: mul s_25_9 s_25_10
        let s_25_11: i128 = ((s_25_9) * (s_25_10));
        // D s_25_12: cast reint s_25_11 -> i64
        let s_25_12: i64 = (s_25_11 as i64);
        // D s_25_13: cast zx s_25_12 -> i
        let s_25_13: i128 = (i128::try_from(s_25_12).unwrap());
        // D s_25_14: write-var return_value <= s_25_13
        fn_state.return_value = s_25_13;
        // N s_25_15: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_26_0: const #1576u : u32
        let s_26_0: u32 = 1576;
        // D s_26_1: read-reg s_26_0:i
        let s_26_1: i128 = {
            let value = state.read_register::<i128>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: write-var return_value <= s_26_1
        fn_state.return_value = s_26_1;
        // N s_26_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_27_0: const #20848u : u32
        let s_27_0: u32 = 20848;
        // D s_27_1: read-reg s_27_0:i
        let s_27_1: i128 = {
            let value = state.read_register::<i128>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: write-var return_value <= s_27_1
        fn_state.return_value = s_27_1;
        // N s_27_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_28_0: const #15624u : u32
        let s_28_0: u32 = 15624;
        // D s_28_1: read-reg s_28_0:i
        let s_28_1: i128 = {
            let value = state.read_register::<i128>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: write-var return_value <= s_28_1
        fn_state.return_value = s_28_1;
        // N s_28_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_29_0: const #90256u : u32
        let s_29_0: u32 = 90256;
        // D s_29_1: read-reg s_29_0:i
        let s_29_1: i128 = {
            let value = state.read_register::<i128>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: write-var return_value <= s_29_1
        fn_state.return_value = s_29_1;
        // N s_29_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_30_0: const #52s : i
        let s_30_0: i128 = 52;
        // D s_30_1: write-var return_value <= s_30_0
        fn_state.return_value = s_30_0;
        // N s_30_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_31_0: const #2s : i
        let s_31_0: i128 = 2;
        // D s_31_1: write-var return_value <= s_31_0
        fn_state.return_value = s_31_0;
        // N s_31_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_32_0: const #4s : i
        let s_32_0: i128 = 4;
        // D s_32_1: write-var return_value <= s_32_0
        fn_state.return_value = s_32_0;
        // N s_32_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_33_0: const #20824u : u32
        let s_33_0: u32 = 20824;
        // D s_33_1: read-reg s_33_0:i
        let s_33_1: i128 = {
            let value = state.read_register::<i128>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: write-var return_value <= s_33_1
        fn_state.return_value = s_33_1;
        // N s_33_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_34_0: const #1s : i
        let s_34_0: i128 = 1;
        // D s_34_1: write-var return_value <= s_34_0
        fn_state.return_value = s_34_0;
        // N s_34_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_35_0: const #11176u : u32
        let s_35_0: u32 = 11176;
        // D s_35_1: read-reg s_35_0:i
        let s_35_1: i128 = {
            let value = state.read_register::<i128>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: write-var return_value <= s_35_1
        fn_state.return_value = s_35_1;
        // N s_35_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call getNumCtxBreakpoints(s_36_0)
        let s_36_1: i128 = getNumCtxBreakpoints(state, tracer, s_36_0);
        // D s_36_2: write-var return_value <= s_36_1
        fn_state.return_value = s_36_1;
        // N s_36_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_37_0: const #11104u : u32
        let s_37_0: u32 = 11104;
        // D s_37_1: read-reg s_37_0:i
        let s_37_1: i128 = {
            let value = state.read_register::<i128>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: write-var return_value <= s_37_1
        fn_state.return_value = s_37_1;
        // N s_37_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call getNumWatchpoints(s_38_0)
        let s_38_1: i128 = getNumWatchpoints(state, tracer, s_38_0);
        // D s_38_2: write-var return_value <= s_38_1
        fn_state.return_value = s_38_1;
        // N s_38_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call getNumBreakpoints(s_39_0)
        let s_39_1: i128 = getNumBreakpoints(state, tracer, s_39_0);
        // D s_39_2: write-var return_value <= s_39_1
        fn_state.return_value = s_39_1;
        // N s_39_3: jump b20
        return block_20(state, tracer, fn_state);
    }
}
