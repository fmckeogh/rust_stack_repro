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
use Mk_ID_AA64ISAR0_EL1_Type::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TME::*;
use u_get_SCR_EL3_Type_TME::*;
use common::*;
pub fn u__get_ID_AA64ISAR0_EL1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        gs_37590: bool,
        gs_37587: bool,
        gs_37588: bool,
        gs_37591: bool,
        gs_37589: bool,
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
        // D s_0_2: read-var tmp.0:struct
        let s_0_2: u64 = fn_state.tmp._0;
        // C s_0_3: const #64s : i
        let s_0_3: i128 = 64;
        // C s_0_4: const #15u : u8
        let s_0_4: u8 = 15;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 4u16);
        // D s_0_6: bits-cast zx s_0_5 -> bv length s_0_3
        let s_0_6: Bits = s_0_5.zero_extend(s_0_3);
        // D s_0_7: cast reint s_0_6 -> u64
        let s_0_7: u64 = (s_0_6.value() as u64);
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 64u16);
        // D s_0_9: not s_0_8
        let s_0_9: Bits = !s_0_8;
        // D s_0_10: cast reint s_0_9 -> u64
        let s_0_10: u64 = (s_0_9.value() as u64);
        // D s_0_11: cast zx s_0_2 -> bv
        let s_0_11: Bits = Bits::new(s_0_2 as u128, 64u16);
        // D s_0_12: cast zx s_0_10 -> bv
        let s_0_12: Bits = Bits::new(s_0_10 as u128, 64u16);
        // D s_0_13: and s_0_11 s_0_12
        let s_0_13: Bits = ((s_0_11) & (s_0_12));
        // D s_0_14: cast reint s_0_13 -> u64
        let s_0_14: u64 = (s_0_13.value() as u64);
        // D s_0_15: call Mk_ID_AA64ISAR0_EL1_Type(s_0_14)
        let s_0_15: ProductType5c790c8ef59cc8b2 = Mk_ID_AA64ISAR0_EL1_Type(
            state,
            tracer,
            s_0_14,
        );
        // D s_0_16: write-var tmp <= s_0_15
        fn_state.tmp = s_0_15;
        // C s_0_17: const #16975u : u32
        let s_0_17: u32 = 16975;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: u8 = {
            let value = state.read_register::<u8>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 2u16);
        // C s_0_20: const #432u : u32
        let s_0_20: u32 = 432;
        // D s_0_21: read-reg s_0_20:u8
        let s_0_21: u8 = {
            let value = state.read_register::<u8>(s_0_20 as isize);
            tracer.read_register(s_0_20 as isize, value);
            value
        };
        // D s_0_22: cast zx s_0_21 -> bv
        let s_0_22: Bits = Bits::new(s_0_21 as u128, 2u16);
        // D s_0_23: cmp-eq s_0_19 s_0_22
        let s_0_23: bool = ((s_0_19) == (s_0_22));
        // N s_0_24: branch s_0_23 b18 b1
        if s_0_23 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #440u : u32
        let s_1_3: u32 = 440;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // D s_1_7: write-var gs#37587 <= s_1_6
        fn_state.gs_37587 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_2_0: read-var gs#37587:u8
        let s_2_0: bool = fn_state.gs_37587;
        // N s_2_1: branch s_2_0 b17 b3
        if s_2_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#37588 <= s_3_0
        fn_state.gs_37588 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_4_0: read-var gs#37588:u8
        let s_4_0: bool = fn_state.gs_37588;
        // N s_4_1: branch s_4_0 b16 b5
        if s_4_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_5_0: const #16975u : u32
        let s_5_0: u32 = 16975;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 2u16);
        // C s_5_3: const #440u : u32
        let s_5_3: u32 = 440;
        // D s_5_4: read-reg s_5_3:u8
        let s_5_4: u8 = {
            let value = state.read_register::<u8>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cmp-eq s_5_2 s_5_5
        let s_5_6: bool = ((s_5_2) == (s_5_5));
        // N s_5_7: branch s_5_6 b15 b6
        if s_5_6 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#37589 <= s_6_0
        fn_state.gs_37589 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_7_0: read-var gs#37589:u8
        let s_7_0: bool = fn_state.gs_37589;
        // N s_7_1: branch s_7_0 b14 b8
        if s_7_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#37590 <= s_8_0
        fn_state.gs_37590 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_9_0: read-var gs#37590:u8
        let s_9_0: bool = fn_state.gs_37590;
        // D s_9_1: write-var gs#37591 <= s_9_0
        fn_state.gs_37591 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_10_0: read-var gs#37591:u8
        let s_10_0: bool = fn_state.gs_37591;
        // N s_10_1: branch s_10_0 b13 b11
        if s_10_0 {
            return block_13(state, tracer, fn_state);
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
        // D s_12_0: read-var tmp:struct
        let s_12_0: ProductType5c790c8ef59cc8b2 = fn_state.tmp;
        // N s_12_1: return s_12_0
        return s_12_0;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_13_0: read-var tmp.0:struct
        let s_13_0: u64 = fn_state.tmp._0;
        // C s_13_1: const #64s : i
        let s_13_1: i128 = 64;
        // C s_13_2: const #251658240u : u28
        let s_13_2: u32 = 251658240;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 28u16);
        // D s_13_4: bits-cast zx s_13_3 -> bv length s_13_1
        let s_13_4: Bits = s_13_3.zero_extend(s_13_1);
        // D s_13_5: cast reint s_13_4 -> u64
        let s_13_5: u64 = (s_13_4.value() as u64);
        // D s_13_6: cast zx s_13_5 -> bv
        let s_13_6: Bits = Bits::new(s_13_5 as u128, 64u16);
        // D s_13_7: not s_13_6
        let s_13_7: Bits = !s_13_6;
        // D s_13_8: cast reint s_13_7 -> u64
        let s_13_8: u64 = (s_13_7.value() as u64);
        // D s_13_9: cast zx s_13_0 -> bv
        let s_13_9: Bits = Bits::new(s_13_0 as u128, 64u16);
        // D s_13_10: cast zx s_13_8 -> bv
        let s_13_10: Bits = Bits::new(s_13_8 as u128, 64u16);
        // D s_13_11: and s_13_9 s_13_10
        let s_13_11: Bits = ((s_13_9) & (s_13_10));
        // D s_13_12: cast reint s_13_11 -> u64
        let s_13_12: u64 = (s_13_11.value() as u64);
        // D s_13_13: call Mk_ID_AA64ISAR0_EL1_Type(s_13_12)
        let s_13_13: ProductType5c790c8ef59cc8b2 = Mk_ID_AA64ISAR0_EL1_Type(
            state,
            tracer,
            s_13_12,
        );
        // D s_13_14: write-var tmp <= s_13_13
        fn_state.tmp = s_13_13;
        // N s_13_15: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_14_0: const #102552u : u32
        let s_14_0: u32 = 102552;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_HCR_EL2_Type_TME(s_14_1)
        let s_14_2: bool = u_get_HCR_EL2_Type_TME(state, tracer, s_14_1);
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #0u : u8
        let s_14_4: bool = false;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // D s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // D s_14_7: write-var gs#37590 <= s_14_6
        fn_state.gs_37590 = s_14_6;
        // N s_14_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call EL2Enabled(s_15_0)
        let s_15_1: bool = EL2Enabled(state, tracer, s_15_0);
        // D s_15_2: write-var gs#37589 <= s_15_1
        fn_state.gs_37589 = s_15_1;
        // N s_15_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#37591 <= s_16_0
        fn_state.gs_37591 = s_16_0;
        // N s_16_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_17_0: const #90704u : u32
        let s_17_0: u32 = 90704;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_SCR_EL3_Type_TME(s_17_1)
        let s_17_2: bool = u_get_SCR_EL3_Type_TME(state, tracer, s_17_1);
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // C s_17_4: const #0u : u8
        let s_17_4: bool = false;
        // C s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 1u16);
        // D s_17_6: cmp-eq s_17_3 s_17_5
        let s_17_6: bool = ((s_17_3) == (s_17_5));
        // D s_17_7: write-var gs#37588 <= s_17_6
        fn_state.gs_37588 = s_17_6;
        // N s_17_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#37587 <= s_18_0
        fn_state.gs_37587 = s_18_0;
        // N s_18_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
