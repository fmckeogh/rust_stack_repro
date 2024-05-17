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
use ConstrainUnpredictableBool::*;
use CurrentSVL_read::*;
use common::*;
pub fn ZAvector_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    index: i128,
    width: i128,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_22653: bool,
        widthshadow_387: i128,
        index: i128,
        width: i128,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        index,
        width,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var width:i
        let s_0_0: i128 = fn_state.width;
        // D s_0_1: write-var widthshadow#387 <= s_0_0
        fn_state.widthshadow_387 = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call CurrentSVL_read(s_0_2)
        let s_0_3: i64 = CurrentSVL_read(state, tracer, s_0_2);
        // S s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_5: read-var widthshadow#387:i
        let s_0_5: i128 = fn_state.widthshadow_387;
        // D s_0_6: cmp-eq s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) == (s_0_4));
        // N s_0_7: assert s_0_6
        let s_0_7: () = assert!(s_0_6);
        // C s_0_8: const #0s : i
        let s_0_8: i128 = 0;
        // D s_0_9: read-var index:i
        let s_0_9: i128 = fn_state.index;
        // D s_0_10: cmp-ge s_0_9 s_0_8
        let s_0_10: bool = ((s_0_9) >= (s_0_8));
        // N s_0_11: branch s_0_10 b5 b1
        if s_0_10 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#22653 <= s_1_0
        fn_state.gs_22653 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#22653:u8
        let s_2_0: bool = fn_state.gs_22653;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #54u : u32
        let s_2_2: u32 = 54;
        // S s_2_3: call ConstrainUnpredictableBool(s_2_2)
        let s_2_3: bool = ConstrainUnpredictableBool(state, tracer, s_2_2);
        // N s_2_4: branch s_2_3 b4 b3
        if s_2_3 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #23952u : u32
        let s_3_0: u32 = 23952;
        // D s_3_1: read-reg s_3_0:[u2048; 256]
        let s_3_1: [u64; 256usize] = {
            let value = state.read_register::<[u64; 256usize]>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: read-var index:i
        let s_3_2: i128 = fn_state.index;
        // D s_3_3: read-element s_3_1[s_3_2]
        let s_3_3: u64 = s_3_1[(s_3_2) as usize];
        // C s_3_4: const #1s : i
        let s_3_4: i128 = 1;
        // D s_3_5: read-var widthshadow#387:i
        let s_3_5: i128 = fn_state.widthshadow_387;
        // D s_3_6: sub s_3_5 s_3_4
        let s_3_6: i128 = ((s_3_5) - (s_3_4));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // C s_3_8: const #0s : i
        let s_3_8: i128 = 0;
        // D s_3_9: cast zx s_3_3 -> bv
        let s_3_9: Bits = Bits::new(s_3_3 as u128, 2048u16);
        // D s_3_10: cast zx s_3_7 -> i
        let s_3_10: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_11: read-var value_name:bv
        let s_3_11: Bits = fn_state.value_name;
        // D s_3_12: sub s_3_10 s_3_8
        let s_3_12: i128 = ((s_3_10) - (s_3_8));
        // C s_3_13: const #1u : u64
        let s_3_13: u64 = 1;
        // C s_3_14: cast zx s_3_13 -> bv
        let s_3_14: Bits = Bits::new(s_3_13 as u128, 64u16);
        // D s_3_15: lsl s_3_14 s_3_12
        let s_3_15: Bits = s_3_14 << s_3_12;
        // D s_3_16: sub s_3_15 s_3_14
        let s_3_16: Bits = ((s_3_15) - (s_3_14));
        // D s_3_17: and s_3_11 s_3_16
        let s_3_17: Bits = ((s_3_11) & (s_3_16));
        // D s_3_18: lsl s_3_17 s_3_8
        let s_3_18: Bits = s_3_17 << s_3_8;
        // D s_3_19: lsl s_3_16 s_3_8
        let s_3_19: Bits = s_3_16 << s_3_8;
        // D s_3_20: cmpl s_3_19
        let s_3_20: Bits = !s_3_19;
        // D s_3_21: and s_3_9 s_3_20
        let s_3_21: Bits = ((s_3_9) & (s_3_20));
        // D s_3_22: or s_3_21 s_3_18
        let s_3_22: Bits = ((s_3_21) | (s_3_18));
        // D s_3_23: cast reint s_3_22 -> u2048
        let s_3_23: u64 = (s_3_22.value() as u64);
        // C s_3_24: const #23952u : u32
        let s_3_24: u32 = 23952;
        // D s_3_25: read-reg s_3_24:[u2048; 256]
        let s_3_25: [u64; 256usize] = {
            let value = state.read_register::<[u64; 256usize]>(s_3_24 as isize);
            tracer.read_register(s_3_24 as isize, value);
            value
        };
        // D s_3_26: read-var index:i
        let s_3_26: i128 = fn_state.index;
        // D s_3_27: mutate-element s_3_25[s_3_26] <= s_3_23
        let s_3_27: [u64; 256usize] = {
            let mut local = s_3_25.clone();
            local[(s_3_26) as usize] = s_3_23;
            local
        };
        // D s_3_28: cast cvt s_3_27 -> [u2048; 0]
        let s_3_28: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_3_27);
        // D s_3_29: cast cvt s_3_28 -> [u2048; 256]
        let s_3_29: [u64; 256usize] = {
            let mut buf = [Default::default(); 256usize];
            buf.copy_from_slice(&s_3_28);
            buf
        };
        // C s_3_30: const #23952u : u32
        let s_3_30: u32 = 23952;
        // N s_3_31: write-reg s_3_30 <= s_3_29
        let s_3_31: () = {
            state.write_register::<[u64; 256usize]>(s_3_30 as isize, s_3_29);
            tracer.write_register(s_3_30 as isize, s_3_29);
        };
        // N s_3_32: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #808u : u32
        let s_4_0: u32 = 808;
        // D s_4_1: read-reg s_4_0:i64
        let s_4_1: i64 = {
            let value = state.read_register::<i64>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: read-var value_name:bv
        let s_4_3: Bits = fn_state.value_name;
        // D s_4_4: bits-cast zx s_4_3 -> bv length s_4_2
        let s_4_4: Bits = s_4_3.zero_extend(s_4_2);
        // D s_4_5: cast reint s_4_4 -> u2048
        let s_4_5: u64 = (s_4_4.value() as u64);
        // C s_4_6: const #23952u : u32
        let s_4_6: u32 = 23952;
        // D s_4_7: read-reg s_4_6:[u2048; 256]
        let s_4_7: [u64; 256usize] = {
            let value = state.read_register::<[u64; 256usize]>(s_4_6 as isize);
            tracer.read_register(s_4_6 as isize, value);
            value
        };
        // D s_4_8: read-var index:i
        let s_4_8: i128 = fn_state.index;
        // D s_4_9: mutate-element s_4_7[s_4_8] <= s_4_5
        let s_4_9: [u64; 256usize] = {
            let mut local = s_4_7.clone();
            local[(s_4_8) as usize] = s_4_5;
            local
        };
        // D s_4_10: cast cvt s_4_9 -> [u2048; 0]
        let s_4_10: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_4_9);
        // D s_4_11: cast cvt s_4_10 -> [u2048; 256]
        let s_4_11: [u64; 256usize] = {
            let mut buf = [Default::default(); 256usize];
            buf.copy_from_slice(&s_4_10);
            buf
        };
        // C s_4_12: const #23952u : u32
        let s_4_12: u32 = 23952;
        // N s_4_13: write-reg s_4_12 <= s_4_11
        let s_4_13: () = {
            state.write_register::<[u64; 256usize]>(s_4_12 as isize, s_4_11);
            tracer.write_register(s_4_12 as isize, s_4_11);
        };
        // N s_4_14: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #8s : i
        let s_5_0: i128 = 8;
        // D s_5_1: read-var widthshadow#387:i
        let s_5_1: i128 = fn_state.widthshadow_387;
        // D s_5_2: div s_5_1 s_5_0
        let s_5_2: i128 = ((s_5_1) / (s_5_0));
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: read-var index:i
        let s_5_5: i128 = fn_state.index;
        // D s_5_6: cmp-lt s_5_5 s_5_4
        let s_5_6: bool = ((s_5_5) < (s_5_4));
        // D s_5_7: write-var gs#22653 <= s_5_6
        fn_state.gs_22653 = s_5_6;
        // N s_5_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
