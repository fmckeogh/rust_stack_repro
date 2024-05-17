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
use Mk_SCR_EL3_Type::*;
use HaveAArch32EL::*;
use common::*;
pub fn u__get_SCR_EL3<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        gs_37629: bool,
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
        // C s_0_3: const #13205680007386497088u : u64
        let s_0_3: u64 = 13205680007386497088;
        // C s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 64u16);
        // C s_0_5: not s_0_4
        let s_0_5: Bits = !s_0_4;
        // C s_0_6: cast reint s_0_5 -> u64
        let s_0_6: u64 = (s_0_5.value() as u64);
        // D s_0_7: cast zx s_0_2 -> bv
        let s_0_7: Bits = Bits::new(s_0_2 as u128, 64u16);
        // C s_0_8: cast zx s_0_6 -> bv
        let s_0_8: Bits = Bits::new(s_0_6 as u128, 64u16);
        // D s_0_9: and s_0_7 s_0_8
        let s_0_9: Bits = ((s_0_7) & (s_0_8));
        // D s_0_10: cast reint s_0_9 -> u64
        let s_0_10: u64 = (s_0_9.value() as u64);
        // C s_0_11: const #64s : i
        let s_0_11: i128 = 64;
        // C s_0_12: const #48u : u8
        let s_0_12: u8 = 48;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 8u16);
        // D s_0_14: bits-cast zx s_0_13 -> bv length s_0_11
        let s_0_14: Bits = s_0_13.zero_extend(s_0_11);
        // D s_0_15: cast reint s_0_14 -> u64
        let s_0_15: u64 = (s_0_14.value() as u64);
        // D s_0_16: cast zx s_0_10 -> bv
        let s_0_16: Bits = Bits::new(s_0_10 as u128, 64u16);
        // D s_0_17: cast zx s_0_15 -> bv
        let s_0_17: Bits = Bits::new(s_0_15 as u128, 64u16);
        // D s_0_18: or s_0_16 s_0_17
        let s_0_18: Bits = ((s_0_16) | (s_0_17));
        // D s_0_19: cast reint s_0_18 -> u64
        let s_0_19: u64 = (s_0_18.value() as u64);
        // D s_0_20: call Mk_SCR_EL3_Type(s_0_19)
        let s_0_20: ProductType5c790c8ef59cc8b2 = Mk_SCR_EL3_Type(state, tracer, s_0_19);
        // D s_0_21: write-var tmp <= s_0_20
        fn_state.tmp = s_0_20;
        // C s_0_22: const #440u : u32
        let s_0_22: u32 = 440;
        // D s_0_23: read-reg s_0_22:u8
        let s_0_23: u8 = {
            let value = state.read_register::<u8>(s_0_22 as isize);
            tracer.read_register(s_0_22 as isize, value);
            value
        };
        // D s_0_24: call HaveAArch32EL(s_0_23)
        let s_0_24: bool = HaveAArch32EL(state, tracer, s_0_23);
        // N s_0_25: branch s_0_24 b6 b1
        if s_0_24 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_1_0: const #432u : u32
        let s_1_0: u32 = 432;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call HaveAArch32EL(s_1_1)
        let s_1_2: bool = HaveAArch32EL(state, tracer, s_1_1);
        // D s_1_3: write-var gs#37629 <= s_1_2
        fn_state.gs_37629 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_2_0: read-var gs#37629:u8
        let s_2_0: bool = fn_state.gs_37629;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b5 b3
        if s_2_1 {
            return block_5(state, tracer, fn_state);
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
        // D s_4_0: read-var tmp:struct
        let s_4_0: ProductType5c790c8ef59cc8b2 = fn_state.tmp;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_5_0: read-var tmp.0:struct
        let s_5_0: u64 = fn_state.tmp._0;
        // C s_5_1: const #64s : i
        let s_5_1: i128 = 64;
        // C s_5_2: const #0u : u8
        let s_5_2: u8 = 0;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: bits-cast zx s_5_3 -> bv length s_5_1
        let s_5_4: Bits = s_5_3.zero_extend(s_5_1);
        // D s_5_5: cast reint s_5_4 -> u64
        let s_5_5: u64 = (s_5_4.value() as u64);
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 64u16);
        // D s_5_7: not s_5_6
        let s_5_7: Bits = !s_5_6;
        // D s_5_8: cast reint s_5_7 -> u64
        let s_5_8: u64 = (s_5_7.value() as u64);
        // D s_5_9: cast zx s_5_0 -> bv
        let s_5_9: Bits = Bits::new(s_5_0 as u128, 64u16);
        // D s_5_10: cast zx s_5_8 -> bv
        let s_5_10: Bits = Bits::new(s_5_8 as u128, 64u16);
        // D s_5_11: and s_5_9 s_5_10
        let s_5_11: Bits = ((s_5_9) & (s_5_10));
        // D s_5_12: cast reint s_5_11 -> u64
        let s_5_12: u64 = (s_5_11.value() as u64);
        // C s_5_13: const #64s : i
        let s_5_13: i128 = 64;
        // C s_5_14: const #1024u : u12
        let s_5_14: u16 = 1024;
        // C s_5_15: cast zx s_5_14 -> bv
        let s_5_15: Bits = Bits::new(s_5_14 as u128, 12u16);
        // D s_5_16: bits-cast zx s_5_15 -> bv length s_5_13
        let s_5_16: Bits = s_5_15.zero_extend(s_5_13);
        // D s_5_17: cast reint s_5_16 -> u64
        let s_5_17: u64 = (s_5_16.value() as u64);
        // D s_5_18: cast zx s_5_12 -> bv
        let s_5_18: Bits = Bits::new(s_5_12 as u128, 64u16);
        // D s_5_19: cast zx s_5_17 -> bv
        let s_5_19: Bits = Bits::new(s_5_17 as u128, 64u16);
        // D s_5_20: or s_5_18 s_5_19
        let s_5_20: Bits = ((s_5_18) | (s_5_19));
        // D s_5_21: cast reint s_5_20 -> u64
        let s_5_21: u64 = (s_5_20.value() as u64);
        // D s_5_22: call Mk_SCR_EL3_Type(s_5_21)
        let s_5_22: ProductType5c790c8ef59cc8b2 = Mk_SCR_EL3_Type(state, tracer, s_5_21);
        // D s_5_23: write-var tmp <= s_5_22
        fn_state.tmp = s_5_22;
        // N s_5_24: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var gs#37629 <= s_6_0
        fn_state.gs_37629 = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
