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
pub fn DBGDTR_EL0_write<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
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
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #"read" : str
        let s_0_2: &'static str = "read";
        // S s_0_3: call __IMPDEF_boolean(s_0_2)
        let s_0_3: bool = u__IMPDEF_boolean(state, tracer, s_0_2);
        // N s_0_4: branch s_0_3 b10 b1
        if s_0_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #"read" : str
        let s_1_0: &'static str = "read";
        // S s_1_1: call __IMPDEF_boolean(s_1_0)
        let s_1_1: bool = u__IMPDEF_boolean(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b9 b2
        if s_1_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r.0:struct
        let s_2_0: u64 = fn_state.r._0;
        // C s_2_1: const #32s : i
        let s_2_1: i128 = 32;
        // C s_2_2: const #32s : i
        let s_2_2: i128 = 32;
        // D s_2_3: cast zx s_2_0 -> bv
        let s_2_3: Bits = Bits::new(s_2_0 as u128, 64u16);
        // D s_2_4: bit-extract s_2_3 s_2_1 s_2_2
        let s_2_4: Bits = (Bits::new(
            ((s_2_3) >> (s_2_1)).value(),
            u16::try_from(s_2_2).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u32
        let s_2_5: u32 = (s_2_4.value() as u32);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #103184u : u32
        let s_2_7: u32 = 103184;
        // D s_2_8: read-reg s_2_7:u64
        let s_2_8: u64 = {
            let value = state.read_register::<u64>(s_2_7 as isize);
            tracer.read_register(s_2_7 as isize, value);
            value
        };
        // D s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 64u16);
        // D s_2_10: cast zx s_2_5 -> bv
        let s_2_10: Bits = Bits::new(s_2_5 as u128, 32u16);
        // C s_2_11: const #31s : i
        let s_2_11: i128 = 31;
        // C s_2_12: const #1u : u64
        let s_2_12: u64 = 1;
        // C s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 64u16);
        // C s_2_14: lsl s_2_13 s_2_11
        let s_2_14: Bits = s_2_13 << s_2_11;
        // C s_2_15: sub s_2_14 s_2_13
        let s_2_15: Bits = ((s_2_14) - (s_2_13));
        // D s_2_16: and s_2_10 s_2_15
        let s_2_16: Bits = ((s_2_10) & (s_2_15));
        // D s_2_17: lsl s_2_16 s_2_6
        let s_2_17: Bits = s_2_16 << s_2_6;
        // C s_2_18: lsl s_2_15 s_2_6
        let s_2_18: Bits = s_2_15 << s_2_6;
        // C s_2_19: cmpl s_2_18
        let s_2_19: Bits = !s_2_18;
        // D s_2_20: and s_2_9 s_2_19
        let s_2_20: Bits = ((s_2_9) & (s_2_19));
        // D s_2_21: or s_2_20 s_2_17
        let s_2_21: Bits = ((s_2_20) | (s_2_17));
        // D s_2_22: cast reint s_2_21 -> u64
        let s_2_22: u64 = (s_2_21.value() as u64);
        // C s_2_23: const #103184u : u32
        let s_2_23: u32 = 103184;
        // N s_2_24: write-reg s_2_23 <= s_2_22
        let s_2_24: () = {
            state.write_register::<u64>(s_2_23 as isize, s_2_22);
            tracer.write_register(s_2_23 as isize, s_2_22);
        };
        // N s_2_25: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #"read" : str
        let s_3_0: &'static str = "read";
        // S s_3_1: call __IMPDEF_boolean(s_3_0)
        let s_3_1: bool = u__IMPDEF_boolean(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b8 b4
        if s_3_1 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #"read" : str
        let s_4_0: &'static str = "read";
        // S s_4_1: call __IMPDEF_boolean(s_4_0)
        let s_4_1: bool = u__IMPDEF_boolean(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b7 b5
        if s_4_1 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var r.0:struct
        let s_5_0: u64 = fn_state.r._0;
        // C s_5_1: const #0s : i
        let s_5_1: i128 = 0;
        // C s_5_2: const #32s : i
        let s_5_2: i128 = 32;
        // D s_5_3: cast zx s_5_0 -> bv
        let s_5_3: Bits = Bits::new(s_5_0 as u128, 64u16);
        // D s_5_4: bit-extract s_5_3 s_5_1 s_5_2
        let s_5_4: Bits = (Bits::new(
            ((s_5_3) >> (s_5_1)).value(),
            u16::try_from(s_5_2).unwrap(),
        ));
        // D s_5_5: cast reint s_5_4 -> u32
        let s_5_5: u32 = (s_5_4.value() as u32);
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // C s_5_7: const #17232u : u32
        let s_5_7: u32 = 17232;
        // D s_5_8: read-reg s_5_7:u64
        let s_5_8: u64 = {
            let value = state.read_register::<u64>(s_5_7 as isize);
            tracer.read_register(s_5_7 as isize, value);
            value
        };
        // D s_5_9: cast zx s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 64u16);
        // D s_5_10: cast zx s_5_5 -> bv
        let s_5_10: Bits = Bits::new(s_5_5 as u128, 32u16);
        // C s_5_11: const #31s : i
        let s_5_11: i128 = 31;
        // C s_5_12: const #1u : u64
        let s_5_12: u64 = 1;
        // C s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 64u16);
        // C s_5_14: lsl s_5_13 s_5_11
        let s_5_14: Bits = s_5_13 << s_5_11;
        // C s_5_15: sub s_5_14 s_5_13
        let s_5_15: Bits = ((s_5_14) - (s_5_13));
        // D s_5_16: and s_5_10 s_5_15
        let s_5_16: Bits = ((s_5_10) & (s_5_15));
        // D s_5_17: lsl s_5_16 s_5_6
        let s_5_17: Bits = s_5_16 << s_5_6;
        // C s_5_18: lsl s_5_15 s_5_6
        let s_5_18: Bits = s_5_15 << s_5_6;
        // C s_5_19: cmpl s_5_18
        let s_5_19: Bits = !s_5_18;
        // D s_5_20: and s_5_9 s_5_19
        let s_5_20: Bits = ((s_5_9) & (s_5_19));
        // D s_5_21: or s_5_20 s_5_17
        let s_5_21: Bits = ((s_5_20) | (s_5_17));
        // D s_5_22: cast reint s_5_21 -> u64
        let s_5_22: u64 = (s_5_21.value() as u64);
        // C s_5_23: const #17232u : u32
        let s_5_23: u32 = 17232;
        // N s_5_24: write-reg s_5_23 <= s_5_22
        let s_5_24: () = {
            state.write_register::<u64>(s_5_23 as isize, s_5_22);
            tracer.write_register(s_5_23 as isize, s_5_22);
        };
        // N s_5_25: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var r:struct
        let s_6_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // C s_6_1: const #11568u : u32
        let s_6_1: u32 = 11568;
        // N s_6_2: write-reg s_6_1 <= s_6_0
        let s_6_2: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_1 as isize, s_6_0);
            tracer.write_register(s_6_1 as isize, s_6_0);
        };
        // N s_6_3: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var r.0:struct
        let s_7_0: u64 = fn_state.r._0;
        // C s_7_1: const #0s : i
        let s_7_1: i128 = 0;
        // C s_7_2: const #32s : i
        let s_7_2: i128 = 32;
        // D s_7_3: cast zx s_7_0 -> bv
        let s_7_3: Bits = Bits::new(s_7_0 as u128, 64u16);
        // D s_7_4: bit-extract s_7_3 s_7_1 s_7_2
        let s_7_4: Bits = (Bits::new(
            ((s_7_3) >> (s_7_1)).value(),
            u16::try_from(s_7_2).unwrap(),
        ));
        // D s_7_5: cast reint s_7_4 -> u32
        let s_7_5: u32 = (s_7_4.value() as u32);
        // C s_7_6: const #0s : i
        let s_7_6: i128 = 0;
        // C s_7_7: const #103184u : u32
        let s_7_7: u32 = 103184;
        // D s_7_8: read-reg s_7_7:u64
        let s_7_8: u64 = {
            let value = state.read_register::<u64>(s_7_7 as isize);
            tracer.read_register(s_7_7 as isize, value);
            value
        };
        // D s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 64u16);
        // D s_7_10: cast zx s_7_5 -> bv
        let s_7_10: Bits = Bits::new(s_7_5 as u128, 32u16);
        // C s_7_11: const #31s : i
        let s_7_11: i128 = 31;
        // C s_7_12: const #1u : u64
        let s_7_12: u64 = 1;
        // C s_7_13: cast zx s_7_12 -> bv
        let s_7_13: Bits = Bits::new(s_7_12 as u128, 64u16);
        // C s_7_14: lsl s_7_13 s_7_11
        let s_7_14: Bits = s_7_13 << s_7_11;
        // C s_7_15: sub s_7_14 s_7_13
        let s_7_15: Bits = ((s_7_14) - (s_7_13));
        // D s_7_16: and s_7_10 s_7_15
        let s_7_16: Bits = ((s_7_10) & (s_7_15));
        // D s_7_17: lsl s_7_16 s_7_6
        let s_7_17: Bits = s_7_16 << s_7_6;
        // C s_7_18: lsl s_7_15 s_7_6
        let s_7_18: Bits = s_7_15 << s_7_6;
        // C s_7_19: cmpl s_7_18
        let s_7_19: Bits = !s_7_18;
        // D s_7_20: and s_7_9 s_7_19
        let s_7_20: Bits = ((s_7_9) & (s_7_19));
        // D s_7_21: or s_7_20 s_7_17
        let s_7_21: Bits = ((s_7_20) | (s_7_17));
        // D s_7_22: cast reint s_7_21 -> u64
        let s_7_22: u64 = (s_7_21.value() as u64);
        // C s_7_23: const #103184u : u32
        let s_7_23: u32 = 103184;
        // N s_7_24: write-reg s_7_23 <= s_7_22
        let s_7_24: () = {
            state.write_register::<u64>(s_7_23 as isize, s_7_22);
            tracer.write_register(s_7_23 as isize, s_7_22);
        };
        // N s_7_25: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var r.0:struct
        let s_8_0: u64 = fn_state.r._0;
        // C s_8_1: const #0s : i
        let s_8_1: i128 = 0;
        // C s_8_2: const #32s : i
        let s_8_2: i128 = 32;
        // D s_8_3: cast zx s_8_0 -> bv
        let s_8_3: Bits = Bits::new(s_8_0 as u128, 64u16);
        // D s_8_4: bit-extract s_8_3 s_8_1 s_8_2
        let s_8_4: Bits = (Bits::new(
            ((s_8_3) >> (s_8_1)).value(),
            u16::try_from(s_8_2).unwrap(),
        ));
        // D s_8_5: cast reint s_8_4 -> u32
        let s_8_5: u32 = (s_8_4.value() as u32);
        // C s_8_6: const #0s : i
        let s_8_6: i128 = 0;
        // C s_8_7: const #103184u : u32
        let s_8_7: u32 = 103184;
        // D s_8_8: read-reg s_8_7:u64
        let s_8_8: u64 = {
            let value = state.read_register::<u64>(s_8_7 as isize);
            tracer.read_register(s_8_7 as isize, value);
            value
        };
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 64u16);
        // D s_8_10: cast zx s_8_5 -> bv
        let s_8_10: Bits = Bits::new(s_8_5 as u128, 32u16);
        // C s_8_11: const #31s : i
        let s_8_11: i128 = 31;
        // C s_8_12: const #1u : u64
        let s_8_12: u64 = 1;
        // C s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 64u16);
        // C s_8_14: lsl s_8_13 s_8_11
        let s_8_14: Bits = s_8_13 << s_8_11;
        // C s_8_15: sub s_8_14 s_8_13
        let s_8_15: Bits = ((s_8_14) - (s_8_13));
        // D s_8_16: and s_8_10 s_8_15
        let s_8_16: Bits = ((s_8_10) & (s_8_15));
        // D s_8_17: lsl s_8_16 s_8_6
        let s_8_17: Bits = s_8_16 << s_8_6;
        // C s_8_18: lsl s_8_15 s_8_6
        let s_8_18: Bits = s_8_15 << s_8_6;
        // C s_8_19: cmpl s_8_18
        let s_8_19: Bits = !s_8_18;
        // D s_8_20: and s_8_9 s_8_19
        let s_8_20: Bits = ((s_8_9) & (s_8_19));
        // D s_8_21: or s_8_20 s_8_17
        let s_8_21: Bits = ((s_8_20) | (s_8_17));
        // D s_8_22: cast reint s_8_21 -> u64
        let s_8_22: u64 = (s_8_21.value() as u64);
        // C s_8_23: const #103184u : u32
        let s_8_23: u32 = 103184;
        // N s_8_24: write-reg s_8_23 <= s_8_22
        let s_8_24: () = {
            state.write_register::<u64>(s_8_23 as isize, s_8_22);
            tracer.write_register(s_8_23 as isize, s_8_22);
        };
        // N s_8_25: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var r.0:struct
        let s_9_0: u64 = fn_state.r._0;
        // C s_9_1: const #32s : i
        let s_9_1: i128 = 32;
        // C s_9_2: const #32s : i
        let s_9_2: i128 = 32;
        // D s_9_3: cast zx s_9_0 -> bv
        let s_9_3: Bits = Bits::new(s_9_0 as u128, 64u16);
        // D s_9_4: bit-extract s_9_3 s_9_1 s_9_2
        let s_9_4: Bits = (Bits::new(
            ((s_9_3) >> (s_9_1)).value(),
            u16::try_from(s_9_2).unwrap(),
        ));
        // D s_9_5: cast reint s_9_4 -> u32
        let s_9_5: u32 = (s_9_4.value() as u32);
        // C s_9_6: const #0s : i
        let s_9_6: i128 = 0;
        // C s_9_7: const #17232u : u32
        let s_9_7: u32 = 17232;
        // D s_9_8: read-reg s_9_7:u64
        let s_9_8: u64 = {
            let value = state.read_register::<u64>(s_9_7 as isize);
            tracer.read_register(s_9_7 as isize, value);
            value
        };
        // D s_9_9: cast zx s_9_8 -> bv
        let s_9_9: Bits = Bits::new(s_9_8 as u128, 64u16);
        // D s_9_10: cast zx s_9_5 -> bv
        let s_9_10: Bits = Bits::new(s_9_5 as u128, 32u16);
        // C s_9_11: const #31s : i
        let s_9_11: i128 = 31;
        // C s_9_12: const #1u : u64
        let s_9_12: u64 = 1;
        // C s_9_13: cast zx s_9_12 -> bv
        let s_9_13: Bits = Bits::new(s_9_12 as u128, 64u16);
        // C s_9_14: lsl s_9_13 s_9_11
        let s_9_14: Bits = s_9_13 << s_9_11;
        // C s_9_15: sub s_9_14 s_9_13
        let s_9_15: Bits = ((s_9_14) - (s_9_13));
        // D s_9_16: and s_9_10 s_9_15
        let s_9_16: Bits = ((s_9_10) & (s_9_15));
        // D s_9_17: lsl s_9_16 s_9_6
        let s_9_17: Bits = s_9_16 << s_9_6;
        // C s_9_18: lsl s_9_15 s_9_6
        let s_9_18: Bits = s_9_15 << s_9_6;
        // C s_9_19: cmpl s_9_18
        let s_9_19: Bits = !s_9_18;
        // D s_9_20: and s_9_9 s_9_19
        let s_9_20: Bits = ((s_9_9) & (s_9_19));
        // D s_9_21: or s_9_20 s_9_17
        let s_9_21: Bits = ((s_9_20) | (s_9_17));
        // D s_9_22: cast reint s_9_21 -> u64
        let s_9_22: u64 = (s_9_21.value() as u64);
        // C s_9_23: const #17232u : u32
        let s_9_23: u32 = 17232;
        // N s_9_24: write-reg s_9_23 <= s_9_22
        let s_9_24: () = {
            state.write_register::<u64>(s_9_23 as isize, s_9_22);
            tracer.write_register(s_9_23 as isize, s_9_22);
        };
        // N s_9_25: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var r.0:struct
        let s_10_0: u64 = fn_state.r._0;
        // C s_10_1: const #32s : i
        let s_10_1: i128 = 32;
        // C s_10_2: const #32s : i
        let s_10_2: i128 = 32;
        // D s_10_3: cast zx s_10_0 -> bv
        let s_10_3: Bits = Bits::new(s_10_0 as u128, 64u16);
        // D s_10_4: bit-extract s_10_3 s_10_1 s_10_2
        let s_10_4: Bits = (Bits::new(
            ((s_10_3) >> (s_10_1)).value(),
            u16::try_from(s_10_2).unwrap(),
        ));
        // D s_10_5: cast reint s_10_4 -> u32
        let s_10_5: u32 = (s_10_4.value() as u32);
        // C s_10_6: const #0s : i
        let s_10_6: i128 = 0;
        // C s_10_7: const #17232u : u32
        let s_10_7: u32 = 17232;
        // D s_10_8: read-reg s_10_7:u64
        let s_10_8: u64 = {
            let value = state.read_register::<u64>(s_10_7 as isize);
            tracer.read_register(s_10_7 as isize, value);
            value
        };
        // D s_10_9: cast zx s_10_8 -> bv
        let s_10_9: Bits = Bits::new(s_10_8 as u128, 64u16);
        // D s_10_10: cast zx s_10_5 -> bv
        let s_10_10: Bits = Bits::new(s_10_5 as u128, 32u16);
        // C s_10_11: const #31s : i
        let s_10_11: i128 = 31;
        // C s_10_12: const #1u : u64
        let s_10_12: u64 = 1;
        // C s_10_13: cast zx s_10_12 -> bv
        let s_10_13: Bits = Bits::new(s_10_12 as u128, 64u16);
        // C s_10_14: lsl s_10_13 s_10_11
        let s_10_14: Bits = s_10_13 << s_10_11;
        // C s_10_15: sub s_10_14 s_10_13
        let s_10_15: Bits = ((s_10_14) - (s_10_13));
        // D s_10_16: and s_10_10 s_10_15
        let s_10_16: Bits = ((s_10_10) & (s_10_15));
        // D s_10_17: lsl s_10_16 s_10_6
        let s_10_17: Bits = s_10_16 << s_10_6;
        // C s_10_18: lsl s_10_15 s_10_6
        let s_10_18: Bits = s_10_15 << s_10_6;
        // C s_10_19: cmpl s_10_18
        let s_10_19: Bits = !s_10_18;
        // D s_10_20: and s_10_9 s_10_19
        let s_10_20: Bits = ((s_10_9) & (s_10_19));
        // D s_10_21: or s_10_20 s_10_17
        let s_10_21: Bits = ((s_10_20) | (s_10_17));
        // D s_10_22: cast reint s_10_21 -> u64
        let s_10_22: u64 = (s_10_21.value() as u64);
        // C s_10_23: const #17232u : u32
        let s_10_23: u32 = 17232;
        // N s_10_24: write-reg s_10_23 <= s_10_22
        let s_10_24: () = {
            state.write_register::<u64>(s_10_23 as isize, s_10_22);
            tracer.write_register(s_10_23 as isize, s_10_22);
        };
        // N s_10_25: jump b3
        return block_3(state, tracer, fn_state);
    }
}
