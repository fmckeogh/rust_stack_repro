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
use common::*;
pub fn AArch64_SS2OutputPASpace<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeb05ce25a107f0c5e,
    ipaspace: u32,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        return_value: u32,
        walkparams: ProductTypeb05ce25a107f0c5e,
        ipaspace: u32,
    }
    let fn_state = FunctionState {
        walkparams,
        ipaspace,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_0_0: read-var ipaspace:u32
        let s_0_0: u32 = fn_state.ipaspace;
        // C s_0_1: const #1u : u32
        let s_0_1: u32 = 1;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b5 b1
        if s_0_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_1_0: read-var walkparams.24:struct
        let s_1_0: bool = fn_state.walkparams._24;
        // D s_1_1: read-var walkparams.19:struct
        let s_1_1: bool = fn_state.walkparams._19;
        // D s_1_2: read-var walkparams.12:struct
        let s_1_2: bool = fn_state.walkparams._12;
        // D s_1_3: read-var walkparams.11:struct
        let s_1_3: bool = fn_state.walkparams._11;
        // D s_1_4: cast zx s_1_2 -> bv
        let s_1_4: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_5: cast zx s_1_3 -> bv
        let s_1_5: Bits = Bits::new(s_1_3 as u128, 1u16);
        // D s_1_6: cast reint s_1_4 -> u128
        let s_1_6: u128 = (s_1_4.value() as u128);
        // D s_1_7: size-of s_1_4
        let s_1_7: u16 = s_1_4.length();
        // D s_1_8: cast reint s_1_5 -> u128
        let s_1_8: u128 = (s_1_5.value() as u128);
        // D s_1_9: size-of s_1_5
        let s_1_9: u16 = s_1_5.length();
        // D s_1_10: lsl s_1_6 s_1_9
        let s_1_10: u128 = s_1_6 << s_1_9;
        // D s_1_11: or s_1_10 s_1_8
        let s_1_11: u128 = ((s_1_10) | (s_1_8));
        // D s_1_12: add s_1_7 s_1_9
        let s_1_12: u16 = (s_1_7 + s_1_9);
        // D s_1_13: create-bits s_1_11 s_1_12
        let s_1_13: Bits = Bits::new(s_1_11, s_1_12);
        // D s_1_14: cast reint s_1_13 -> u8
        let s_1_14: u8 = (s_1_13.value() as u8);
        // D s_1_15: cast zx s_1_1 -> bv
        let s_1_15: Bits = Bits::new(s_1_1 as u128, 1u16);
        // D s_1_16: cast zx s_1_14 -> bv
        let s_1_16: Bits = Bits::new(s_1_14 as u128, 2u16);
        // D s_1_17: cast reint s_1_15 -> u128
        let s_1_17: u128 = (s_1_15.value() as u128);
        // D s_1_18: size-of s_1_15
        let s_1_18: u16 = s_1_15.length();
        // D s_1_19: cast reint s_1_16 -> u128
        let s_1_19: u128 = (s_1_16.value() as u128);
        // D s_1_20: size-of s_1_16
        let s_1_20: u16 = s_1_16.length();
        // D s_1_21: lsl s_1_17 s_1_20
        let s_1_21: u128 = s_1_17 << s_1_20;
        // D s_1_22: or s_1_21 s_1_19
        let s_1_22: u128 = ((s_1_21) | (s_1_19));
        // D s_1_23: add s_1_18 s_1_20
        let s_1_23: u16 = (s_1_18 + s_1_20);
        // D s_1_24: create-bits s_1_22 s_1_23
        let s_1_24: Bits = Bits::new(s_1_22, s_1_23);
        // D s_1_25: cast reint s_1_24 -> u8
        let s_1_25: u8 = (s_1_24.value() as u8);
        // D s_1_26: cast zx s_1_0 -> bv
        let s_1_26: Bits = Bits::new(s_1_0 as u128, 1u16);
        // D s_1_27: cast zx s_1_25 -> bv
        let s_1_27: Bits = Bits::new(s_1_25 as u128, 3u16);
        // D s_1_28: cast reint s_1_26 -> u128
        let s_1_28: u128 = (s_1_26.value() as u128);
        // D s_1_29: size-of s_1_26
        let s_1_29: u16 = s_1_26.length();
        // D s_1_30: cast reint s_1_27 -> u128
        let s_1_30: u128 = (s_1_27.value() as u128);
        // D s_1_31: size-of s_1_27
        let s_1_31: u16 = s_1_27.length();
        // D s_1_32: lsl s_1_28 s_1_31
        let s_1_32: u128 = s_1_28 << s_1_31;
        // D s_1_33: or s_1_32 s_1_30
        let s_1_33: u128 = ((s_1_32) | (s_1_30));
        // D s_1_34: add s_1_29 s_1_31
        let s_1_34: u16 = (s_1_29 + s_1_31);
        // D s_1_35: create-bits s_1_33 s_1_34
        let s_1_35: Bits = Bits::new(s_1_33, s_1_34);
        // D s_1_36: cast reint s_1_35 -> u8
        let s_1_36: u8 = (s_1_35.value() as u8);
        // D s_1_37: cast zx s_1_36 -> bv
        let s_1_37: Bits = Bits::new(s_1_36 as u128, 4u16);
        // C s_1_38: const #0u : u8
        let s_1_38: u8 = 0;
        // C s_1_39: cast zx s_1_38 -> bv
        let s_1_39: Bits = Bits::new(s_1_38 as u128, 4u16);
        // D s_1_40: cmp-eq s_1_37 s_1_39
        let s_1_40: bool = ((s_1_37) == (s_1_39));
        // N s_1_41: branch s_1_40 b4 b2
        if s_1_40 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_2_0: const #0u : u32
        let s_2_0: u32 = 0;
        // D s_2_1: write-var return_value <= s_2_0
        fn_state.return_value = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_3_0: read-var return_value:u32
        let s_3_0: u32 = fn_state.return_value;
        // N s_3_1: return s_3_0
        return s_3_0;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_4_0: const #1u : u32
        let s_4_0: u32 = 1;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_5_0: read-var walkparams.24:struct
        let s_5_0: bool = fn_state.walkparams._24;
        // D s_5_1: read-var walkparams.19:struct
        let s_5_1: bool = fn_state.walkparams._19;
        // D s_5_2: cast zx s_5_0 -> bv
        let s_5_2: Bits = Bits::new(s_5_0 as u128, 1u16);
        // D s_5_3: cast zx s_5_1 -> bv
        let s_5_3: Bits = Bits::new(s_5_1 as u128, 1u16);
        // D s_5_4: cast reint s_5_2 -> u128
        let s_5_4: u128 = (s_5_2.value() as u128);
        // D s_5_5: size-of s_5_2
        let s_5_5: u16 = s_5_2.length();
        // D s_5_6: cast reint s_5_3 -> u128
        let s_5_6: u128 = (s_5_3.value() as u128);
        // D s_5_7: size-of s_5_3
        let s_5_7: u16 = s_5_3.length();
        // D s_5_8: lsl s_5_4 s_5_7
        let s_5_8: u128 = s_5_4 << s_5_7;
        // D s_5_9: or s_5_8 s_5_6
        let s_5_9: u128 = ((s_5_8) | (s_5_6));
        // D s_5_10: add s_5_5 s_5_7
        let s_5_10: u16 = (s_5_5 + s_5_7);
        // D s_5_11: create-bits s_5_9 s_5_10
        let s_5_11: Bits = Bits::new(s_5_9, s_5_10);
        // D s_5_12: cast reint s_5_11 -> u8
        let s_5_12: u8 = (s_5_11.value() as u8);
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 2u16);
        // C s_5_14: const #0u : u8
        let s_5_14: u8 = 0;
        // C s_5_15: cast zx s_5_14 -> bv
        let s_5_15: Bits = Bits::new(s_5_14 as u128, 2u16);
        // D s_5_16: cmp-eq s_5_13 s_5_15
        let s_5_16: bool = ((s_5_13) == (s_5_15));
        // N s_5_17: branch s_5_16 b7 b6
        if s_5_16 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_6_0: const #0u : u32
        let s_6_0: u32 = 0;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #1u : u32
        let s_7_0: u32 = 1;
        // D s_7_1: write-var return_value <= s_7_0
        fn_state.return_value = s_7_0;
        // N s_7_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
