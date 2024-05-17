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
use R_set::*;
use R_read::*;
use CheckAdvSIMDEnabled::*;
use Elem_read::*;
use D_read::*;
use MemU_set::*;
use common::*;
pub fn execute_aarch32_instrs_VST3_1_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    d2: i128,
    d3: i128,
    ebytes: i64,
    index: i64,
    m: i64,
    n: i64,
    register_index: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u32,
        d: i64,
        d2: i128,
        d3: i128,
        ebytes: i64,
        index: i64,
        m: i64,
        n: i64,
        register_index: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        d,
        d2,
        d3,
        ebytes,
        index,
        m,
        n,
        register_index,
        wback,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // D s_1_3: write-var address <= s_1_2
        fn_state.address = s_1_2;
        // D s_1_4: read-var d:i64
        let s_1_4: i64 = fn_state.d;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: call D_read(s_1_5)
        let s_1_6: u64 = D_read(state, tracer, s_1_5);
        // C s_1_7: const #8s : i
        let s_1_7: i128 = 8;
        // D s_1_8: read-var ebytes:i64
        let s_1_8: i64 = fn_state.ebytes;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: mul s_1_7 s_1_9
        let s_1_10: i128 = ((s_1_7) * (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: cast zx s_1_6 -> bv
        let s_1_14: Bits = Bits::new(s_1_6 as u128, 64u16);
        // D s_1_15: read-var index:i64
        let s_1_15: i64 = fn_state.index;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_13 -> i
        let s_1_17: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_18: call Elem_read(s_1_14, s_1_16, s_1_17)
        let s_1_18: Bits = Elem_read(state, tracer, s_1_14, s_1_16, s_1_17);
        // D s_1_19: read-var ebytes:i64
        let s_1_19: i64 = fn_state.ebytes;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: read-var address:u32
        let s_1_21: u32 = fn_state.address;
        // D s_1_22: call MemU_set(s_1_21, s_1_20, s_1_18)
        let s_1_22: () = MemU_set(state, tracer, s_1_21, s_1_20, s_1_18);
        // N s_1_23: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var address:u32
        let s_2_0: u32 = fn_state.address;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 32u16);
        // D s_2_2: read-var ebytes:i64
        let s_2_2: i64 = fn_state.ebytes;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: cast cvt s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 128);
        // D s_2_5: add s_2_1 s_2_4
        let s_2_5: Bits = (s_2_1 + s_2_4);
        // D s_2_6: cast reint s_2_5 -> u32
        let s_2_6: u32 = (s_2_5.value() as u32);
        // D s_2_7: read-var d2:i
        let s_2_7: i128 = fn_state.d2;
        // D s_2_8: call D_read(s_2_7)
        let s_2_8: u64 = D_read(state, tracer, s_2_7);
        // C s_2_9: const #8s : i
        let s_2_9: i128 = 8;
        // D s_2_10: read-var ebytes:i64
        let s_2_10: i64 = fn_state.ebytes;
        // D s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (i128::try_from(s_2_10).unwrap());
        // D s_2_12: mul s_2_9 s_2_11
        let s_2_12: i128 = ((s_2_9) * (s_2_11));
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_15: cast reint s_2_14 -> i64
        let s_2_15: i64 = (s_2_14 as i64);
        // D s_2_16: cast zx s_2_8 -> bv
        let s_2_16: Bits = Bits::new(s_2_8 as u128, 64u16);
        // D s_2_17: read-var index:i64
        let s_2_17: i64 = fn_state.index;
        // D s_2_18: cast zx s_2_17 -> i
        let s_2_18: i128 = (i128::try_from(s_2_17).unwrap());
        // D s_2_19: cast zx s_2_15 -> i
        let s_2_19: i128 = (i128::try_from(s_2_15).unwrap());
        // D s_2_20: call Elem_read(s_2_16, s_2_18, s_2_19)
        let s_2_20: Bits = Elem_read(state, tracer, s_2_16, s_2_18, s_2_19);
        // D s_2_21: read-var ebytes:i64
        let s_2_21: i64 = fn_state.ebytes;
        // D s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_23: call MemU_set(s_2_6, s_2_22, s_2_20)
        let s_2_23: () = MemU_set(state, tracer, s_2_6, s_2_22, s_2_20);
        // N s_2_24: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var ebytes:i64
        let s_3_1: i64 = fn_state.ebytes;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: read-var address:u32
        let s_3_5: u32 = fn_state.address;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 32u16);
        // D s_3_7: cast zx s_3_4 -> i
        let s_3_7: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_8: cast cvt s_3_7 -> bv
        let s_3_8: Bits = Bits::new(s_3_7 as u128, 128);
        // D s_3_9: add s_3_6 s_3_8
        let s_3_9: Bits = (s_3_6 + s_3_8);
        // D s_3_10: cast reint s_3_9 -> u32
        let s_3_10: u32 = (s_3_9.value() as u32);
        // D s_3_11: read-var d3:i
        let s_3_11: i128 = fn_state.d3;
        // D s_3_12: call D_read(s_3_11)
        let s_3_12: u64 = D_read(state, tracer, s_3_11);
        // C s_3_13: const #8s : i
        let s_3_13: i128 = 8;
        // D s_3_14: read-var ebytes:i64
        let s_3_14: i64 = fn_state.ebytes;
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: mul s_3_13 s_3_15
        let s_3_16: i128 = ((s_3_13) * (s_3_15));
        // D s_3_17: cast reint s_3_16 -> i64
        let s_3_17: i64 = (s_3_16 as i64);
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: cast reint s_3_18 -> i64
        let s_3_19: i64 = (s_3_18 as i64);
        // D s_3_20: cast zx s_3_12 -> bv
        let s_3_20: Bits = Bits::new(s_3_12 as u128, 64u16);
        // D s_3_21: read-var index:i64
        let s_3_21: i64 = fn_state.index;
        // D s_3_22: cast zx s_3_21 -> i
        let s_3_22: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_23: cast zx s_3_19 -> i
        let s_3_23: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_24: call Elem_read(s_3_20, s_3_22, s_3_23)
        let s_3_24: Bits = Elem_read(state, tracer, s_3_20, s_3_22, s_3_23);
        // D s_3_25: read-var ebytes:i64
        let s_3_25: i64 = fn_state.ebytes;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: call MemU_set(s_3_10, s_3_26, s_3_24)
        let s_3_27: () = MemU_set(state, tracer, s_3_10, s_3_26, s_3_24);
        // N s_3_28: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var wback:u8
        let s_4_0: bool = fn_state.wback;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var register_index:u8
        let s_6_0: bool = fn_state.register_index;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var n:i64
        let s_7_0: i64 = fn_state.n;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call R_read(s_7_1)
        let s_7_2: u32 = R_read(state, tracer, s_7_1);
        // C s_7_3: const #3s : i
        let s_7_3: i128 = 3;
        // D s_7_4: read-var ebytes:i64
        let s_7_4: i64 = fn_state.ebytes;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: mul s_7_3 s_7_5
        let s_7_6: i128 = ((s_7_3) * (s_7_5));
        // D s_7_7: cast reint s_7_6 -> i64
        let s_7_7: i64 = (s_7_6 as i64);
        // D s_7_8: cast zx s_7_2 -> bv
        let s_7_8: Bits = Bits::new(s_7_2 as u128, 32u16);
        // D s_7_9: cast zx s_7_7 -> i
        let s_7_9: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_10: cast cvt s_7_9 -> bv
        let s_7_10: Bits = Bits::new(s_7_9 as u128, 128);
        // D s_7_11: add s_7_8 s_7_10
        let s_7_11: Bits = (s_7_8 + s_7_10);
        // D s_7_12: cast reint s_7_11 -> u32
        let s_7_12: u32 = (s_7_11.value() as u32);
        // D s_7_13: read-var n:i64
        let s_7_13: i64 = fn_state.n;
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_15: call R_set(s_7_14, s_7_12)
        let s_7_15: () = R_set(state, tracer, s_7_14, s_7_12);
        // N s_7_16: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var n:i64
        let s_8_0: i64 = fn_state.n;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call R_read(s_8_1)
        let s_8_2: u32 = R_read(state, tracer, s_8_1);
        // D s_8_3: read-var m:i64
        let s_8_3: i64 = fn_state.m;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: call R_read(s_8_4)
        let s_8_5: u32 = R_read(state, tracer, s_8_4);
        // D s_8_6: cast zx s_8_2 -> bv
        let s_8_6: Bits = Bits::new(s_8_2 as u128, 32u16);
        // D s_8_7: cast zx s_8_5 -> bv
        let s_8_7: Bits = Bits::new(s_8_5 as u128, 32u16);
        // D s_8_8: add s_8_6 s_8_7
        let s_8_8: Bits = (s_8_6 + s_8_7);
        // D s_8_9: cast reint s_8_8 -> u32
        let s_8_9: u32 = (s_8_8.value() as u32);
        // D s_8_10: read-var n:i64
        let s_8_10: i64 = fn_state.n;
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: call R_set(s_8_11, s_8_9)
        let s_8_12: () = R_set(state, tracer, s_8_11, s_8_9);
        // N s_8_13: return
        return;
    }
}
