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
use u_get_GCSPR_EL1_Type_PTR::*;
use u_get_GCSPR_EL2_Type_PTR::*;
use u_get_GCSPR_EL0_Type_PTR::*;
use u_get_GCSPR_EL3_Type_PTR::*;
use common::*;
pub fn GetCurrentGCSPointer<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_22954: (),
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        ga_17782: u8,
        ptr: u64,
        gs_22954: (),
    }
    let fn_state = FunctionState {
        gs_22954,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var ga#17782 <= s_0_1
        fn_state.ga_17782 = s_0_1;
        // D s_0_3: read-var ga#17782:u8
        let s_0_3: u8 = fn_state.ga_17782;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // C s_0_5: const #448u : u32
        let s_0_5: u32 = 448;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: u8 = {
            let value = state.read_register::<u8>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // D s_0_8: cmp-eq s_0_4 s_0_7
        let s_0_8: bool = ((s_0_4) == (s_0_7));
        // D s_0_9: not s_0_8
        let s_0_9: bool = !s_0_8;
        // N s_0_10: branch s_0_9 b3 b1
        if s_0_9 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_1_0: const #20840u : u32
        let s_1_0: u32 = 20840;
        // D s_1_1: read-reg s_1_0:struct
        let s_1_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call _get_GCSPR_EL0_Type_PTR(s_1_1)
        let s_1_2: u64 = u_get_GCSPR_EL0_Type_PTR(state, tracer, s_1_1);
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 61u16);
        // C s_1_4: const #0u : u8
        let s_1_4: u8 = 0;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 3u16);
        // D s_1_6: cast reint s_1_3 -> u128
        let s_1_6: u128 = (s_1_3.value() as u128);
        // D s_1_7: size-of s_1_3
        let s_1_7: u16 = s_1_3.length();
        // C s_1_8: cast reint s_1_5 -> u128
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
        // D s_1_14: cast reint s_1_13 -> u64
        let s_1_14: u64 = (s_1_13.value() as u64);
        // D s_1_15: write-var ptr <= s_1_14
        fn_state.ptr = s_1_14;
        // N s_1_16: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var ptr:u64
        let s_2_0: u64 = fn_state.ptr;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_3_0: read-var ga#17782:u8
        let s_3_0: u8 = fn_state.ga_17782;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #440u : u32
        let s_3_2: u32 = 440;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // D s_3_6: not s_3_5
        let s_3_6: bool = !s_3_5;
        // N s_3_7: branch s_3_6 b5 b4
        if s_3_6 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_4_0: const #21912u : u32
        let s_4_0: u32 = 21912;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_GCSPR_EL1_Type_PTR(s_4_1)
        let s_4_2: u64 = u_get_GCSPR_EL1_Type_PTR(state, tracer, s_4_1);
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 61u16);
        // C s_4_4: const #0u : u8
        let s_4_4: u8 = 0;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 3u16);
        // D s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // C s_4_8: cast reint s_4_5 -> u128
        let s_4_8: u128 = (s_4_5.value() as u128);
        // D s_4_9: size-of s_4_5
        let s_4_9: u16 = s_4_5.length();
        // D s_4_10: lsl s_4_6 s_4_9
        let s_4_10: u128 = s_4_6 << s_4_9;
        // D s_4_11: or s_4_10 s_4_8
        let s_4_11: u128 = ((s_4_10) | (s_4_8));
        // D s_4_12: add s_4_7 s_4_9
        let s_4_12: u16 = (s_4_7 + s_4_9);
        // D s_4_13: create-bits s_4_11 s_4_12
        let s_4_13: Bits = Bits::new(s_4_11, s_4_12);
        // D s_4_14: cast reint s_4_13 -> u64
        let s_4_14: u64 = (s_4_13.value() as u64);
        // D s_4_15: write-var ptr <= s_4_14
        fn_state.ptr = s_4_14;
        // N s_4_16: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_5_0: read-var ga#17782:u8
        let s_5_0: u8 = fn_state.ga_17782;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #432u : u32
        let s_5_2: u32 = 432;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // D s_5_6: not s_5_5
        let s_5_6: bool = !s_5_5;
        // N s_5_7: branch s_5_6 b7 b6
        if s_5_6 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_6_0: const #10216u : u32
        let s_6_0: u32 = 10216;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call _get_GCSPR_EL2_Type_PTR(s_6_1)
        let s_6_2: u64 = u_get_GCSPR_EL2_Type_PTR(state, tracer, s_6_1);
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 61u16);
        // C s_6_4: const #0u : u8
        let s_6_4: u8 = 0;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 3u16);
        // D s_6_6: cast reint s_6_3 -> u128
        let s_6_6: u128 = (s_6_3.value() as u128);
        // D s_6_7: size-of s_6_3
        let s_6_7: u16 = s_6_3.length();
        // C s_6_8: cast reint s_6_5 -> u128
        let s_6_8: u128 = (s_6_5.value() as u128);
        // D s_6_9: size-of s_6_5
        let s_6_9: u16 = s_6_5.length();
        // D s_6_10: lsl s_6_6 s_6_9
        let s_6_10: u128 = s_6_6 << s_6_9;
        // D s_6_11: or s_6_10 s_6_8
        let s_6_11: u128 = ((s_6_10) | (s_6_8));
        // D s_6_12: add s_6_7 s_6_9
        let s_6_12: u16 = (s_6_7 + s_6_9);
        // D s_6_13: create-bits s_6_11 s_6_12
        let s_6_13: Bits = Bits::new(s_6_11, s_6_12);
        // D s_6_14: cast reint s_6_13 -> u64
        let s_6_14: u64 = (s_6_13.value() as u64);
        // D s_6_15: write-var ptr <= s_6_14
        fn_state.ptr = s_6_14;
        // N s_6_16: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var ga#17782:u8
        let s_7_0: u8 = fn_state.ga_17782;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #424u : u32
        let s_7_2: u32 = 424;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // D s_7_6: not s_7_5
        let s_7_6: bool = !s_7_5;
        // N s_7_7: branch s_7_6 b9 b8
        if s_7_6 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_8_0: const #18248u : u32
        let s_8_0: u32 = 18248;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_GCSPR_EL3_Type_PTR(s_8_1)
        let s_8_2: u64 = u_get_GCSPR_EL3_Type_PTR(state, tracer, s_8_1);
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 61u16);
        // C s_8_4: const #0u : u8
        let s_8_4: u8 = 0;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 3u16);
        // D s_8_6: cast reint s_8_3 -> u128
        let s_8_6: u128 = (s_8_3.value() as u128);
        // D s_8_7: size-of s_8_3
        let s_8_7: u16 = s_8_3.length();
        // C s_8_8: cast reint s_8_5 -> u128
        let s_8_8: u128 = (s_8_5.value() as u128);
        // D s_8_9: size-of s_8_5
        let s_8_9: u16 = s_8_5.length();
        // D s_8_10: lsl s_8_6 s_8_9
        let s_8_10: u128 = s_8_6 << s_8_9;
        // D s_8_11: or s_8_10 s_8_8
        let s_8_11: u128 = ((s_8_10) | (s_8_8));
        // D s_8_12: add s_8_7 s_8_9
        let s_8_12: u16 = (s_8_7 + s_8_9);
        // D s_8_13: create-bits s_8_11 s_8_12
        let s_8_13: Bits = Bits::new(s_8_11, s_8_12);
        // D s_8_14: cast reint s_8_13 -> u64
        let s_8_14: u64 = (s_8_13.value() as u64);
        // D s_8_15: write-var ptr <= s_8_14
        fn_state.ptr = s_8_14;
        // N s_8_16: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_9_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}