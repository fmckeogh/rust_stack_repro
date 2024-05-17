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
use Mk_FPEXC32_EL2_Type::*;
use common::*;
pub fn u__get_FPEXC32_EL2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        tmp: ProductType5c790c8ef59cc8b2,
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var tmp <= s_0_0
        fn_state.tmp = s_0_0;
        // C s_0_2: const #"IMPLEMENTED_trapping of Invalid Operation floating-point exceptions" : str
        let s_0_2: &'static str = "IMPLEMENTED_trapping of Invalid Operation floating-point exceptions";
        // S s_0_3: call __IMPDEF_boolean(s_0_2)
        let s_0_3: bool = u__IMPDEF_boolean(state, tracer, s_0_2);
        // S s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b21 b1
        if s_0_4 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_2_0: const #"IMPLEMENTED_trapping of Divide by Zero floating-point exceptions" : str
        let s_2_0: &'static str = "IMPLEMENTED_trapping of Divide by Zero floating-point exceptions";
        // S s_2_1: call __IMPDEF_boolean(s_2_0)
        let s_2_1: bool = u__IMPDEF_boolean(state, tracer, s_2_0);
        // S s_2_2: not s_2_1
        let s_2_2: bool = !s_2_1;
        // N s_2_3: branch s_2_2 b20 b3
        if s_2_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_4_0: const #"IMPLEMENTED_trapping of Overflow floating-point exceptions" : str
        let s_4_0: &'static str = "IMPLEMENTED_trapping of Overflow floating-point exceptions";
        // S s_4_1: call __IMPDEF_boolean(s_4_0)
        let s_4_1: bool = u__IMPDEF_boolean(state, tracer, s_4_0);
        // S s_4_2: not s_4_1
        let s_4_2: bool = !s_4_1;
        // N s_4_3: branch s_4_2 b19 b5
        if s_4_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_6_0: const #"IMPLEMENTED_trapping of Underflow floating-point exceptions" : str
        let s_6_0: &'static str = "IMPLEMENTED_trapping of Underflow floating-point exceptions";
        // S s_6_1: call __IMPDEF_boolean(s_6_0)
        let s_6_1: bool = u__IMPDEF_boolean(state, tracer, s_6_0);
        // S s_6_2: not s_6_1
        let s_6_2: bool = !s_6_1;
        // N s_6_3: branch s_6_2 b18 b7
        if s_6_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_8_0: const #"IMPLEMENTED_trapping of Inexact floating-point exceptions" : str
        let s_8_0: &'static str = "IMPLEMENTED_trapping of Inexact floating-point exceptions";
        // S s_8_1: call __IMPDEF_boolean(s_8_0)
        let s_8_1: bool = u__IMPDEF_boolean(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // N s_8_3: branch s_8_2 b17 b9
        if s_8_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_10_0: read-var tmp.0:struct
        let s_10_0: u64 = fn_state.tmp._0;
        // C s_10_1: const #18446744072031828064u : u64
        let s_10_1: u64 = 18446744072031828064;
        // C s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 64u16);
        // C s_10_3: not s_10_2
        let s_10_3: Bits = !s_10_2;
        // C s_10_4: cast reint s_10_3 -> u64
        let s_10_4: u64 = (s_10_3.value() as u64);
        // D s_10_5: cast zx s_10_0 -> bv
        let s_10_5: Bits = Bits::new(s_10_0 as u128, 64u16);
        // C s_10_6: cast zx s_10_4 -> bv
        let s_10_6: Bits = Bits::new(s_10_4 as u128, 64u16);
        // D s_10_7: and s_10_5 s_10_6
        let s_10_7: Bits = ((s_10_5) & (s_10_6));
        // D s_10_8: cast reint s_10_7 -> u64
        let s_10_8: u64 = (s_10_7.value() as u64);
        // C s_10_9: const #64s : i
        let s_10_9: i128 = 64;
        // C s_10_10: const #1792u : u12
        let s_10_10: u16 = 1792;
        // C s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 12u16);
        // D s_10_12: bits-cast zx s_10_11 -> bv length s_10_9
        let s_10_12: Bits = s_10_11.zero_extend(s_10_9);
        // D s_10_13: cast reint s_10_12 -> u64
        let s_10_13: u64 = (s_10_12.value() as u64);
        // D s_10_14: cast zx s_10_8 -> bv
        let s_10_14: Bits = Bits::new(s_10_8 as u128, 64u16);
        // D s_10_15: cast zx s_10_13 -> bv
        let s_10_15: Bits = Bits::new(s_10_13 as u128, 64u16);
        // D s_10_16: or s_10_14 s_10_15
        let s_10_16: Bits = ((s_10_14) | (s_10_15));
        // D s_10_17: cast reint s_10_16 -> u64
        let s_10_17: u64 = (s_10_16.value() as u64);
        // D s_10_18: call Mk_FPEXC32_EL2_Type(s_10_17)
        let s_10_18: ProductType5c790c8ef59cc8b2 = Mk_FPEXC32_EL2_Type(
            state,
            tracer,
            s_10_17,
        );
        // D s_10_19: write-var tmp <= s_10_18
        fn_state.tmp = s_10_18;
        // C s_10_20: const #"IMPLEMENTED_trapping of Input Denormal floating-point exceptions" : str
        let s_10_20: &'static str = "IMPLEMENTED_trapping of Input Denormal floating-point exceptions";
        // S s_10_21: call __IMPDEF_boolean(s_10_20)
        let s_10_21: bool = u__IMPDEF_boolean(state, tracer, s_10_20);
        // S s_10_22: not s_10_21
        let s_10_22: bool = !s_10_21;
        // N s_10_23: branch s_10_22 b16 b11
        if s_10_22 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_12_0: const #"IMPLEMENTED_trapping of floating-point exceptions" : str
        let s_12_0: &'static str = "IMPLEMENTED_trapping of floating-point exceptions";
        // S s_12_1: call __IMPDEF_boolean(s_12_0)
        let s_12_1: bool = u__IMPDEF_boolean(state, tracer, s_12_0);
        // S s_12_2: not s_12_1
        let s_12_2: bool = !s_12_1;
        // N s_12_3: branch s_12_2 b15 b13
        if s_12_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_14_0: read-var tmp:struct
        let s_14_0: ProductType5c790c8ef59cc8b2 = fn_state.tmp;
        // N s_14_1: return s_14_0
        return s_14_0;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_15_0: read-var tmp.0:struct
        let s_15_0: u64 = fn_state.tmp._0;
        // C s_15_1: const #64s : i
        let s_15_1: i128 = 64;
        // C s_15_2: const #67108864u : u28
        let s_15_2: u32 = 67108864;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 28u16);
        // D s_15_4: bits-cast zx s_15_3 -> bv length s_15_1
        let s_15_4: Bits = s_15_3.zero_extend(s_15_1);
        // D s_15_5: cast reint s_15_4 -> u64
        let s_15_5: u64 = (s_15_4.value() as u64);
        // D s_15_6: cast zx s_15_5 -> bv
        let s_15_6: Bits = Bits::new(s_15_5 as u128, 64u16);
        // D s_15_7: not s_15_6
        let s_15_7: Bits = !s_15_6;
        // D s_15_8: cast reint s_15_7 -> u64
        let s_15_8: u64 = (s_15_7.value() as u64);
        // D s_15_9: cast zx s_15_0 -> bv
        let s_15_9: Bits = Bits::new(s_15_0 as u128, 64u16);
        // D s_15_10: cast zx s_15_8 -> bv
        let s_15_10: Bits = Bits::new(s_15_8 as u128, 64u16);
        // D s_15_11: and s_15_9 s_15_10
        let s_15_11: Bits = ((s_15_9) & (s_15_10));
        // D s_15_12: cast reint s_15_11 -> u64
        let s_15_12: u64 = (s_15_11.value() as u64);
        // D s_15_13: call Mk_FPEXC32_EL2_Type(s_15_12)
        let s_15_13: ProductType5c790c8ef59cc8b2 = Mk_FPEXC32_EL2_Type(
            state,
            tracer,
            s_15_12,
        );
        // D s_15_14: write-var tmp <= s_15_13
        fn_state.tmp = s_15_13;
        // N s_15_15: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_16_0: read-var tmp.0:struct
        let s_16_0: u64 = fn_state.tmp._0;
        // C s_16_1: const #64s : i
        let s_16_1: i128 = 64;
        // C s_16_2: const #128u : u8
        let s_16_2: u8 = 128;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 8u16);
        // D s_16_4: bits-cast zx s_16_3 -> bv length s_16_1
        let s_16_4: Bits = s_16_3.zero_extend(s_16_1);
        // D s_16_5: cast reint s_16_4 -> u64
        let s_16_5: u64 = (s_16_4.value() as u64);
        // D s_16_6: cast zx s_16_5 -> bv
        let s_16_6: Bits = Bits::new(s_16_5 as u128, 64u16);
        // D s_16_7: not s_16_6
        let s_16_7: Bits = !s_16_6;
        // D s_16_8: cast reint s_16_7 -> u64
        let s_16_8: u64 = (s_16_7.value() as u64);
        // D s_16_9: cast zx s_16_0 -> bv
        let s_16_9: Bits = Bits::new(s_16_0 as u128, 64u16);
        // D s_16_10: cast zx s_16_8 -> bv
        let s_16_10: Bits = Bits::new(s_16_8 as u128, 64u16);
        // D s_16_11: and s_16_9 s_16_10
        let s_16_11: Bits = ((s_16_9) & (s_16_10));
        // D s_16_12: cast reint s_16_11 -> u64
        let s_16_12: u64 = (s_16_11.value() as u64);
        // D s_16_13: call Mk_FPEXC32_EL2_Type(s_16_12)
        let s_16_13: ProductType5c790c8ef59cc8b2 = Mk_FPEXC32_EL2_Type(
            state,
            tracer,
            s_16_12,
        );
        // D s_16_14: write-var tmp <= s_16_13
        fn_state.tmp = s_16_13;
        // N s_16_15: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_17_0: read-var tmp.0:struct
        let s_17_0: u64 = fn_state.tmp._0;
        // C s_17_1: const #64s : i
        let s_17_1: i128 = 64;
        // C s_17_2: const #16u : u8
        let s_17_2: u8 = 16;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 8u16);
        // D s_17_4: bits-cast zx s_17_3 -> bv length s_17_1
        let s_17_4: Bits = s_17_3.zero_extend(s_17_1);
        // D s_17_5: cast reint s_17_4 -> u64
        let s_17_5: u64 = (s_17_4.value() as u64);
        // D s_17_6: cast zx s_17_5 -> bv
        let s_17_6: Bits = Bits::new(s_17_5 as u128, 64u16);
        // D s_17_7: not s_17_6
        let s_17_7: Bits = !s_17_6;
        // D s_17_8: cast reint s_17_7 -> u64
        let s_17_8: u64 = (s_17_7.value() as u64);
        // D s_17_9: cast zx s_17_0 -> bv
        let s_17_9: Bits = Bits::new(s_17_0 as u128, 64u16);
        // D s_17_10: cast zx s_17_8 -> bv
        let s_17_10: Bits = Bits::new(s_17_8 as u128, 64u16);
        // D s_17_11: and s_17_9 s_17_10
        let s_17_11: Bits = ((s_17_9) & (s_17_10));
        // D s_17_12: cast reint s_17_11 -> u64
        let s_17_12: u64 = (s_17_11.value() as u64);
        // D s_17_13: call Mk_FPEXC32_EL2_Type(s_17_12)
        let s_17_13: ProductType5c790c8ef59cc8b2 = Mk_FPEXC32_EL2_Type(
            state,
            tracer,
            s_17_12,
        );
        // D s_17_14: write-var tmp <= s_17_13
        fn_state.tmp = s_17_13;
        // N s_17_15: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_18_0: read-var tmp.0:struct
        let s_18_0: u64 = fn_state.tmp._0;
        // C s_18_1: const #64s : i
        let s_18_1: i128 = 64;
        // C s_18_2: const #8u : u8
        let s_18_2: u8 = 8;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 4u16);
        // D s_18_4: bits-cast zx s_18_3 -> bv length s_18_1
        let s_18_4: Bits = s_18_3.zero_extend(s_18_1);
        // D s_18_5: cast reint s_18_4 -> u64
        let s_18_5: u64 = (s_18_4.value() as u64);
        // D s_18_6: cast zx s_18_5 -> bv
        let s_18_6: Bits = Bits::new(s_18_5 as u128, 64u16);
        // D s_18_7: not s_18_6
        let s_18_7: Bits = !s_18_6;
        // D s_18_8: cast reint s_18_7 -> u64
        let s_18_8: u64 = (s_18_7.value() as u64);
        // D s_18_9: cast zx s_18_0 -> bv
        let s_18_9: Bits = Bits::new(s_18_0 as u128, 64u16);
        // D s_18_10: cast zx s_18_8 -> bv
        let s_18_10: Bits = Bits::new(s_18_8 as u128, 64u16);
        // D s_18_11: and s_18_9 s_18_10
        let s_18_11: Bits = ((s_18_9) & (s_18_10));
        // D s_18_12: cast reint s_18_11 -> u64
        let s_18_12: u64 = (s_18_11.value() as u64);
        // D s_18_13: call Mk_FPEXC32_EL2_Type(s_18_12)
        let s_18_13: ProductType5c790c8ef59cc8b2 = Mk_FPEXC32_EL2_Type(
            state,
            tracer,
            s_18_12,
        );
        // D s_18_14: write-var tmp <= s_18_13
        fn_state.tmp = s_18_13;
        // N s_18_15: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_19_0: read-var tmp.0:struct
        let s_19_0: u64 = fn_state.tmp._0;
        // C s_19_1: const #64s : i
        let s_19_1: i128 = 64;
        // C s_19_2: const #4u : u8
        let s_19_2: u8 = 4;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 4u16);
        // D s_19_4: bits-cast zx s_19_3 -> bv length s_19_1
        let s_19_4: Bits = s_19_3.zero_extend(s_19_1);
        // D s_19_5: cast reint s_19_4 -> u64
        let s_19_5: u64 = (s_19_4.value() as u64);
        // D s_19_6: cast zx s_19_5 -> bv
        let s_19_6: Bits = Bits::new(s_19_5 as u128, 64u16);
        // D s_19_7: not s_19_6
        let s_19_7: Bits = !s_19_6;
        // D s_19_8: cast reint s_19_7 -> u64
        let s_19_8: u64 = (s_19_7.value() as u64);
        // D s_19_9: cast zx s_19_0 -> bv
        let s_19_9: Bits = Bits::new(s_19_0 as u128, 64u16);
        // D s_19_10: cast zx s_19_8 -> bv
        let s_19_10: Bits = Bits::new(s_19_8 as u128, 64u16);
        // D s_19_11: and s_19_9 s_19_10
        let s_19_11: Bits = ((s_19_9) & (s_19_10));
        // D s_19_12: cast reint s_19_11 -> u64
        let s_19_12: u64 = (s_19_11.value() as u64);
        // D s_19_13: call Mk_FPEXC32_EL2_Type(s_19_12)
        let s_19_13: ProductType5c790c8ef59cc8b2 = Mk_FPEXC32_EL2_Type(
            state,
            tracer,
            s_19_12,
        );
        // D s_19_14: write-var tmp <= s_19_13
        fn_state.tmp = s_19_13;
        // N s_19_15: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_20_0: read-var tmp.0:struct
        let s_20_0: u64 = fn_state.tmp._0;
        // C s_20_1: const #64s : i
        let s_20_1: i128 = 64;
        // C s_20_2: const #2u : u8
        let s_20_2: u8 = 2;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 4u16);
        // D s_20_4: bits-cast zx s_20_3 -> bv length s_20_1
        let s_20_4: Bits = s_20_3.zero_extend(s_20_1);
        // D s_20_5: cast reint s_20_4 -> u64
        let s_20_5: u64 = (s_20_4.value() as u64);
        // D s_20_6: cast zx s_20_5 -> bv
        let s_20_6: Bits = Bits::new(s_20_5 as u128, 64u16);
        // D s_20_7: not s_20_6
        let s_20_7: Bits = !s_20_6;
        // D s_20_8: cast reint s_20_7 -> u64
        let s_20_8: u64 = (s_20_7.value() as u64);
        // D s_20_9: cast zx s_20_0 -> bv
        let s_20_9: Bits = Bits::new(s_20_0 as u128, 64u16);
        // D s_20_10: cast zx s_20_8 -> bv
        let s_20_10: Bits = Bits::new(s_20_8 as u128, 64u16);
        // D s_20_11: and s_20_9 s_20_10
        let s_20_11: Bits = ((s_20_9) & (s_20_10));
        // D s_20_12: cast reint s_20_11 -> u64
        let s_20_12: u64 = (s_20_11.value() as u64);
        // D s_20_13: call Mk_FPEXC32_EL2_Type(s_20_12)
        let s_20_13: ProductType5c790c8ef59cc8b2 = Mk_FPEXC32_EL2_Type(
            state,
            tracer,
            s_20_12,
        );
        // D s_20_14: write-var tmp <= s_20_13
        fn_state.tmp = s_20_13;
        // N s_20_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_21_0: read-var tmp.0:struct
        let s_21_0: u64 = fn_state.tmp._0;
        // C s_21_1: const #64s : i
        let s_21_1: i128 = 64;
        // C s_21_2: const #1u : u8
        let s_21_2: u8 = 1;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 4u16);
        // D s_21_4: bits-cast zx s_21_3 -> bv length s_21_1
        let s_21_4: Bits = s_21_3.zero_extend(s_21_1);
        // D s_21_5: cast reint s_21_4 -> u64
        let s_21_5: u64 = (s_21_4.value() as u64);
        // D s_21_6: cast zx s_21_5 -> bv
        let s_21_6: Bits = Bits::new(s_21_5 as u128, 64u16);
        // D s_21_7: not s_21_6
        let s_21_7: Bits = !s_21_6;
        // D s_21_8: cast reint s_21_7 -> u64
        let s_21_8: u64 = (s_21_7.value() as u64);
        // D s_21_9: cast zx s_21_0 -> bv
        let s_21_9: Bits = Bits::new(s_21_0 as u128, 64u16);
        // D s_21_10: cast zx s_21_8 -> bv
        let s_21_10: Bits = Bits::new(s_21_8 as u128, 64u16);
        // D s_21_11: and s_21_9 s_21_10
        let s_21_11: Bits = ((s_21_9) & (s_21_10));
        // D s_21_12: cast reint s_21_11 -> u64
        let s_21_12: u64 = (s_21_11.value() as u64);
        // D s_21_13: call Mk_FPEXC32_EL2_Type(s_21_12)
        let s_21_13: ProductType5c790c8ef59cc8b2 = Mk_FPEXC32_EL2_Type(
            state,
            tracer,
            s_21_12,
        );
        // D s_21_14: write-var tmp <= s_21_13
        fn_state.tmp = s_21_13;
        // N s_21_15: jump b2
        return block_2(state, tracer, fn_state);
    }
}
