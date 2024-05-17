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
use CurrentVL_read::*;
use u__id::*;
use ConstrainUnpredictableBool::*;
use common::*;
pub fn P_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i128,
    width: i128,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_4134: bool,
        widthshadow_47: i128,
        VL: i64,
        gs_4140: bool,
        gs_4127: bool,
        n: i128,
        width: i128,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        n,
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
        // D s_0_1: write-var widthshadow#47 <= s_0_0
        fn_state.widthshadow_47 = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call CurrentVL_read(s_0_2)
        let s_0_3: i64 = CurrentVL_read(state, tracer, s_0_2);
        // D s_0_4: write-var VL <= s_0_3
        fn_state.VL = s_0_3;
        // C s_0_5: const #0s : i
        let s_0_5: i128 = 0;
        // D s_0_6: read-var n:i
        let s_0_6: i128 = fn_state.n;
        // D s_0_7: cmp-ge s_0_6 s_0_5
        let s_0_7: bool = ((s_0_6) >= (s_0_5));
        // N s_0_8: branch s_0_7 b11 b1
        if s_0_7 {
            return block_11(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#4127 <= s_1_0
        fn_state.gs_4127 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#4127:u8
        let s_2_0: bool = fn_state.gs_4127;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #8s : i
        let s_2_2: i128 = 8;
        // D s_2_3: read-var VL:i64
        let s_2_3: i64 = fn_state.VL;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: div s_2_4 s_2_2
        let s_2_5: i128 = ((s_2_4) / (s_2_2));
        // D s_2_6: cast reint s_2_5 -> i64
        let s_2_6: i64 = (s_2_5 as i64);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_8: read-var widthshadow#47:i
        let s_2_8: i128 = fn_state.widthshadow_47;
        // D s_2_9: cmp-eq s_2_8 s_2_7
        let s_2_9: bool = ((s_2_8) == (s_2_7));
        // N s_2_10: assert s_2_9
        let s_2_10: () = assert!(s_2_9);
        // C s_2_11: const #50u : u32
        let s_2_11: u32 = 50;
        // S s_2_12: call ConstrainUnpredictableBool(s_2_11)
        let s_2_12: bool = ConstrainUnpredictableBool(state, tracer, s_2_11);
        // N s_2_13: branch s_2_12 b7 b3
        if s_2_12 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i
        let s_3_0: i128 = fn_state.n;
        // D s_3_1: call __id(s_3_0)
        let s_3_1: i128 = u__id(state, tracer, s_3_0);
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // C s_3_3: const #0s : i
        let s_3_3: i128 = 0;
        // D s_3_4: cast zx s_3_2 -> i
        let s_3_4: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_5: cmp-le s_3_3 s_3_4
        let s_3_5: bool = ((s_3_3) <= (s_3_4));
        // N s_3_6: branch s_3_5 b6 b4
        if s_3_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#4134 <= s_4_0
        fn_state.gs_4134 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#4134:u8
        let s_5_0: bool = fn_state.gs_4134;
        // N s_5_1: assert s_5_0
        let s_5_1: () = assert!(s_5_0);
        // C s_5_2: const #17736u : u32
        let s_5_2: u32 = 17736;
        // D s_5_3: read-reg s_5_2:[u256; 16]
        let s_5_3: [u64; 16usize] = {
            let value = state.read_register::<[u64; 16usize]>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: read-var n:i
        let s_5_4: i128 = fn_state.n;
        // D s_5_5: read-element s_5_3[s_5_4]
        let s_5_5: u64 = s_5_3[(s_5_4) as usize];
        // C s_5_6: const #1s : i
        let s_5_6: i128 = 1;
        // D s_5_7: read-var widthshadow#47:i
        let s_5_7: i128 = fn_state.widthshadow_47;
        // D s_5_8: sub s_5_7 s_5_6
        let s_5_8: i128 = ((s_5_7) - (s_5_6));
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // C s_5_10: const #0s : i
        let s_5_10: i128 = 0;
        // D s_5_11: cast zx s_5_5 -> bv
        let s_5_11: Bits = Bits::new(s_5_5 as u128, 256u16);
        // D s_5_12: cast zx s_5_9 -> i
        let s_5_12: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_13: read-var value_name:bv
        let s_5_13: Bits = fn_state.value_name;
        // D s_5_14: sub s_5_12 s_5_10
        let s_5_14: i128 = ((s_5_12) - (s_5_10));
        // C s_5_15: const #1u : u64
        let s_5_15: u64 = 1;
        // C s_5_16: cast zx s_5_15 -> bv
        let s_5_16: Bits = Bits::new(s_5_15 as u128, 64u16);
        // D s_5_17: lsl s_5_16 s_5_14
        let s_5_17: Bits = s_5_16 << s_5_14;
        // D s_5_18: sub s_5_17 s_5_16
        let s_5_18: Bits = ((s_5_17) - (s_5_16));
        // D s_5_19: and s_5_13 s_5_18
        let s_5_19: Bits = ((s_5_13) & (s_5_18));
        // D s_5_20: lsl s_5_19 s_5_10
        let s_5_20: Bits = s_5_19 << s_5_10;
        // D s_5_21: lsl s_5_18 s_5_10
        let s_5_21: Bits = s_5_18 << s_5_10;
        // D s_5_22: cmpl s_5_21
        let s_5_22: Bits = !s_5_21;
        // D s_5_23: and s_5_11 s_5_22
        let s_5_23: Bits = ((s_5_11) & (s_5_22));
        // D s_5_24: or s_5_23 s_5_20
        let s_5_24: Bits = ((s_5_23) | (s_5_20));
        // D s_5_25: cast reint s_5_24 -> u256
        let s_5_25: u64 = (s_5_24.value() as u64);
        // C s_5_26: const #17736u : u32
        let s_5_26: u32 = 17736;
        // D s_5_27: read-reg s_5_26:[u256; 16]
        let s_5_27: [u64; 16usize] = {
            let value = state.read_register::<[u64; 16usize]>(s_5_26 as isize);
            tracer.read_register(s_5_26 as isize, value);
            value
        };
        // D s_5_28: read-var n:i
        let s_5_28: i128 = fn_state.n;
        // D s_5_29: mutate-element s_5_27[s_5_28] <= s_5_25
        let s_5_29: [u64; 16usize] = {
            let mut local = s_5_27.clone();
            local[(s_5_28) as usize] = s_5_25;
            local
        };
        // D s_5_30: cast cvt s_5_29 -> [u256; 0]
        let s_5_30: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_5_29);
        // D s_5_31: cast cvt s_5_30 -> [u256; 16]
        let s_5_31: [u64; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_5_30);
            buf
        };
        // C s_5_32: const #17736u : u32
        let s_5_32: u32 = 17736;
        // N s_5_33: write-reg s_5_32 <= s_5_31
        let s_5_33: () = {
            state.write_register::<[u64; 16usize]>(s_5_32 as isize, s_5_31);
            tracer.write_register(s_5_32 as isize, s_5_31);
        };
        // N s_5_34: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var n:i
        let s_6_0: i128 = fn_state.n;
        // D s_6_1: call __id(s_6_0)
        let s_6_1: i128 = u__id(state, tracer, s_6_0);
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // C s_6_3: const #16s : i
        let s_6_3: i128 = 16;
        // D s_6_4: cast zx s_6_2 -> i
        let s_6_4: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_5: cmp-lt s_6_4 s_6_3
        let s_6_5: bool = ((s_6_4) < (s_6_3));
        // D s_6_6: write-var gs#4134 <= s_6_5
        fn_state.gs_4134 = s_6_5;
        // N s_6_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var n:i
        let s_7_0: i128 = fn_state.n;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // C s_7_3: const #0s : i
        let s_7_3: i128 = 0;
        // D s_7_4: cast zx s_7_2 -> i
        let s_7_4: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_5: cmp-le s_7_3 s_7_4
        let s_7_5: bool = ((s_7_3) <= (s_7_4));
        // N s_7_6: branch s_7_5 b10 b8
        if s_7_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#4140 <= s_8_0
        fn_state.gs_4140 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#4140:u8
        let s_9_0: bool = fn_state.gs_4140;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // C s_9_2: const #816u : u32
        let s_9_2: u32 = 816;
        // D s_9_3: read-reg s_9_2:i64
        let s_9_3: i64 = {
            let value = state.read_register::<i64>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: read-var value_name:bv
        let s_9_5: Bits = fn_state.value_name;
        // D s_9_6: bits-cast zx s_9_5 -> bv length s_9_4
        let s_9_6: Bits = s_9_5.zero_extend(s_9_4);
        // D s_9_7: cast reint s_9_6 -> u256
        let s_9_7: u64 = (s_9_6.value() as u64);
        // C s_9_8: const #17736u : u32
        let s_9_8: u32 = 17736;
        // D s_9_9: read-reg s_9_8:[u256; 16]
        let s_9_9: [u64; 16usize] = {
            let value = state.read_register::<[u64; 16usize]>(s_9_8 as isize);
            tracer.read_register(s_9_8 as isize, value);
            value
        };
        // D s_9_10: read-var n:i
        let s_9_10: i128 = fn_state.n;
        // D s_9_11: mutate-element s_9_9[s_9_10] <= s_9_7
        let s_9_11: [u64; 16usize] = {
            let mut local = s_9_9.clone();
            local[(s_9_10) as usize] = s_9_7;
            local
        };
        // D s_9_12: cast cvt s_9_11 -> [u256; 0]
        let s_9_12: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_9_11);
        // D s_9_13: cast cvt s_9_12 -> [u256; 16]
        let s_9_13: [u64; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_9_12);
            buf
        };
        // C s_9_14: const #17736u : u32
        let s_9_14: u32 = 17736;
        // N s_9_15: write-reg s_9_14 <= s_9_13
        let s_9_15: () = {
            state.write_register::<[u64; 16usize]>(s_9_14 as isize, s_9_13);
            tracer.write_register(s_9_14 as isize, s_9_13);
        };
        // N s_9_16: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var n:i
        let s_10_0: i128 = fn_state.n;
        // D s_10_1: call __id(s_10_0)
        let s_10_1: i128 = u__id(state, tracer, s_10_0);
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // C s_10_3: const #16s : i
        let s_10_3: i128 = 16;
        // D s_10_4: cast zx s_10_2 -> i
        let s_10_4: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_5: cmp-lt s_10_4 s_10_3
        let s_10_5: bool = ((s_10_4) < (s_10_3));
        // D s_10_6: write-var gs#4140 <= s_10_5
        fn_state.gs_4140 = s_10_5;
        // N s_10_7: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #31s : i
        let s_11_0: i128 = 31;
        // D s_11_1: read-var n:i
        let s_11_1: i128 = fn_state.n;
        // D s_11_2: cmp-le s_11_1 s_11_0
        let s_11_2: bool = ((s_11_1) <= (s_11_0));
        // D s_11_3: write-var gs#4127 <= s_11_2
        fn_state.gs_4127 = s_11_2;
        // N s_11_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
