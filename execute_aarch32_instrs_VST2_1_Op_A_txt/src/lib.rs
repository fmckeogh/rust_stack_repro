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
use R_read::*;
use CheckAdvSIMDEnabled::*;
use MemU_set::*;
use R_set::*;
use AArch32_Abort::*;
use IsAligned__1::*;
use CreateAccDescASIMD::*;
use Elem_read::*;
use D_read::*;
use AlignmentFault::*;
use common::*;
pub fn execute_aarch32_instrs_VST2_1_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    alignment: i64,
    d: i64,
    d2: i128,
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
        accdesc: ProductType9878976b5bcce9c9,
        alignment: i64,
        d: i64,
        d2: i128,
        ebytes: i64,
        index: i64,
        m: i64,
        n: i64,
        register_index: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        alignment,
        d,
        d2,
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
        // C s_1_4: const #1u : u32
        let s_1_4: u32 = 1;
        // C s_1_5: const #0u : u8
        let s_1_5: bool = false;
        // C s_1_6: const #0u : u8
        let s_1_6: bool = false;
        // S s_1_7: call CreateAccDescASIMD(s_1_4, s_1_5, s_1_6)
        let s_1_7: ProductType9878976b5bcce9c9 = CreateAccDescASIMD(
            state,
            tracer,
            s_1_4,
            s_1_5,
            s_1_6,
        );
        // D s_1_8: write-var accdesc <= s_1_7
        fn_state.accdesc = s_1_7;
        // D s_1_9: read-var address:u32
        let s_1_9: u32 = fn_state.address;
        // D s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 32u16);
        // D s_1_11: read-var alignment:i64
        let s_1_11: i64 = fn_state.alignment;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call IsAligned__1(s_1_10, s_1_12)
        let s_1_13: bool = IsAligned__1(state, tracer, s_1_10, s_1_12);
        // D s_1_14: not s_1_13
        let s_1_14: bool = !s_1_13;
        // N s_1_15: branch s_1_14 b10 b2
        if s_1_14 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var d:i64
        let s_3_0: i64 = fn_state.d;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call D_read(s_3_1)
        let s_3_2: u64 = D_read(state, tracer, s_3_1);
        // C s_3_3: const #8s : i
        let s_3_3: i128 = 8;
        // D s_3_4: read-var ebytes:i64
        let s_3_4: i64 = fn_state.ebytes;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: mul s_3_3 s_3_5
        let s_3_6: i128 = ((s_3_3) * (s_3_5));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: cast zx s_3_2 -> bv
        let s_3_10: Bits = Bits::new(s_3_2 as u128, 64u16);
        // D s_3_11: read-var index:i64
        let s_3_11: i64 = fn_state.index;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: cast zx s_3_9 -> i
        let s_3_13: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_14: call Elem_read(s_3_10, s_3_12, s_3_13)
        let s_3_14: Bits = Elem_read(state, tracer, s_3_10, s_3_12, s_3_13);
        // D s_3_15: read-var ebytes:i64
        let s_3_15: i64 = fn_state.ebytes;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: read-var address:u32
        let s_3_17: u32 = fn_state.address;
        // D s_3_18: call MemU_set(s_3_17, s_3_16, s_3_14)
        let s_3_18: () = MemU_set(state, tracer, s_3_17, s_3_16, s_3_14);
        // N s_3_19: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var address:u32
        let s_4_0: u32 = fn_state.address;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 32u16);
        // D s_4_2: read-var ebytes:i64
        let s_4_2: i64 = fn_state.ebytes;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: cast cvt s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 128);
        // D s_4_5: add s_4_1 s_4_4
        let s_4_5: Bits = (s_4_1 + s_4_4);
        // D s_4_6: cast reint s_4_5 -> u32
        let s_4_6: u32 = (s_4_5.value() as u32);
        // D s_4_7: read-var d2:i
        let s_4_7: i128 = fn_state.d2;
        // D s_4_8: call D_read(s_4_7)
        let s_4_8: u64 = D_read(state, tracer, s_4_7);
        // C s_4_9: const #8s : i
        let s_4_9: i128 = 8;
        // D s_4_10: read-var ebytes:i64
        let s_4_10: i64 = fn_state.ebytes;
        // D s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (i128::try_from(s_4_10).unwrap());
        // D s_4_12: mul s_4_9 s_4_11
        let s_4_12: i128 = ((s_4_9) * (s_4_11));
        // D s_4_13: cast reint s_4_12 -> i64
        let s_4_13: i64 = (s_4_12 as i64);
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: cast zx s_4_8 -> bv
        let s_4_16: Bits = Bits::new(s_4_8 as u128, 64u16);
        // D s_4_17: read-var index:i64
        let s_4_17: i64 = fn_state.index;
        // D s_4_18: cast zx s_4_17 -> i
        let s_4_18: i128 = (i128::try_from(s_4_17).unwrap());
        // D s_4_19: cast zx s_4_15 -> i
        let s_4_19: i128 = (i128::try_from(s_4_15).unwrap());
        // D s_4_20: call Elem_read(s_4_16, s_4_18, s_4_19)
        let s_4_20: Bits = Elem_read(state, tracer, s_4_16, s_4_18, s_4_19);
        // D s_4_21: read-var ebytes:i64
        let s_4_21: i64 = fn_state.ebytes;
        // D s_4_22: cast zx s_4_21 -> i
        let s_4_22: i128 = (i128::try_from(s_4_21).unwrap());
        // D s_4_23: call MemU_set(s_4_6, s_4_22, s_4_20)
        let s_4_23: () = MemU_set(state, tracer, s_4_6, s_4_22, s_4_20);
        // N s_4_24: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var wback:u8
        let s_5_0: bool = fn_state.wback;
        // N s_5_1: branch s_5_0 b7 b6
        if s_5_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var register_index:u8
        let s_7_0: bool = fn_state.register_index;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
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
        // C s_8_3: const #2s : i
        let s_8_3: i128 = 2;
        // D s_8_4: read-var ebytes:i64
        let s_8_4: i64 = fn_state.ebytes;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: mul s_8_3 s_8_5
        let s_8_6: i128 = ((s_8_3) * (s_8_5));
        // D s_8_7: cast reint s_8_6 -> i64
        let s_8_7: i64 = (s_8_6 as i64);
        // D s_8_8: cast zx s_8_2 -> bv
        let s_8_8: Bits = Bits::new(s_8_2 as u128, 32u16);
        // D s_8_9: cast zx s_8_7 -> i
        let s_8_9: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_10: cast cvt s_8_9 -> bv
        let s_8_10: Bits = Bits::new(s_8_9 as u128, 128);
        // D s_8_11: add s_8_8 s_8_10
        let s_8_11: Bits = (s_8_8 + s_8_10);
        // D s_8_12: cast reint s_8_11 -> u32
        let s_8_12: u32 = (s_8_11.value() as u32);
        // D s_8_13: read-var n:i64
        let s_8_13: i64 = fn_state.n;
        // D s_8_14: cast zx s_8_13 -> i
        let s_8_14: i128 = (i128::try_from(s_8_13).unwrap());
        // D s_8_15: call R_set(s_8_14, s_8_12)
        let s_8_15: () = R_set(state, tracer, s_8_14, s_8_12);
        // N s_8_16: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var n:i64
        let s_9_0: i64 = fn_state.n;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call R_read(s_9_1)
        let s_9_2: u32 = R_read(state, tracer, s_9_1);
        // D s_9_3: read-var m:i64
        let s_9_3: i64 = fn_state.m;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: call R_read(s_9_4)
        let s_9_5: u32 = R_read(state, tracer, s_9_4);
        // D s_9_6: cast zx s_9_2 -> bv
        let s_9_6: Bits = Bits::new(s_9_2 as u128, 32u16);
        // D s_9_7: cast zx s_9_5 -> bv
        let s_9_7: Bits = Bits::new(s_9_5 as u128, 32u16);
        // D s_9_8: add s_9_6 s_9_7
        let s_9_8: Bits = (s_9_6 + s_9_7);
        // D s_9_9: cast reint s_9_8 -> u32
        let s_9_9: u32 = (s_9_8.value() as u32);
        // D s_9_10: read-var n:i64
        let s_9_10: i64 = fn_state.n;
        // D s_9_11: cast zx s_9_10 -> i
        let s_9_11: i128 = (i128::try_from(s_9_10).unwrap());
        // D s_9_12: call R_set(s_9_11, s_9_9)
        let s_9_12: () = R_set(state, tracer, s_9_11, s_9_9);
        // N s_9_13: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var accdesc:struct
        let s_10_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_10_1: call AlignmentFault(s_10_0)
        let s_10_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_10_0);
        // D s_10_2: read-var address:u32
        let s_10_2: u32 = fn_state.address;
        // D s_10_3: call AArch32_Abort(s_10_2, s_10_1)
        let s_10_3: () = AArch32_Abort(state, tracer, s_10_2, s_10_1);
        // N s_10_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
