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
pub fn execute_aarch32_instrs_VST2_m_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    alignment: i64,
    d: i64,
    d2: i128,
    ebytes: i64,
    elements: i128,
    m: i64,
    n: i64,
    pairs: i64,
    register_index: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        gs_320974: i64,
        address: u32,
        accdesc: ProductType9878976b5bcce9c9,
        gs_320980: i64,
        alignment: i64,
        d: i64,
        d2: i128,
        ebytes: i64,
        elements: i128,
        m: i64,
        n: i64,
        pairs: i64,
        register_index: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        alignment,
        d,
        d2,
        ebytes,
        elements,
        m,
        n,
        pairs,
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
        // N s_1_15: branch s_1_14 b16 b2
        if s_1_14 {
            return block_16(state, tracer, fn_state);
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
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var pairs:i64
        let s_3_2: i64 = fn_state.pairs;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#320974 <= s_3_5
        fn_state.gs_320974 = s_3_5;
        // D s_3_7: write-var r <= s_3_0
        fn_state.r = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:i64
        let s_4_0: i64 = fn_state.r;
        // D s_4_1: read-var gs#320974:i64
        let s_4_1: i64 = fn_state.gs_320974;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b11 b5
        if s_4_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var elements:i
        let s_5_2: i128 = fn_state.elements;
        // D s_5_3: sub s_5_2 s_5_1
        let s_5_3: i128 = ((s_5_2) - (s_5_1));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: write-var gs#320980 <= s_5_4
        fn_state.gs_320980 = s_5_4;
        // D s_5_6: write-var e <= s_5_0
        fn_state.e = s_5_0;
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#320980:i64
        let s_6_1: i64 = fn_state.gs_320980;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b10 b7
        if s_6_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var d:i64
        let s_7_0: i64 = fn_state.d;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var r:i64
        let s_7_2: i64 = fn_state.r;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: add s_7_1 s_7_3
        let s_7_4: i128 = (s_7_1 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: call D_read(s_7_6)
        let s_7_7: u64 = D_read(state, tracer, s_7_6);
        // C s_7_8: const #8s : i
        let s_7_8: i128 = 8;
        // D s_7_9: read-var ebytes:i64
        let s_7_9: i64 = fn_state.ebytes;
        // D s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_11: mul s_7_8 s_7_10
        let s_7_11: i128 = ((s_7_8) * (s_7_10));
        // D s_7_12: cast reint s_7_11 -> i64
        let s_7_12: i64 = (s_7_11 as i64);
        // D s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_14: cast reint s_7_13 -> i64
        let s_7_14: i64 = (s_7_13 as i64);
        // D s_7_15: cast zx s_7_7 -> bv
        let s_7_15: Bits = Bits::new(s_7_7 as u128, 64u16);
        // D s_7_16: read-var e:i64
        let s_7_16: i64 = fn_state.e;
        // D s_7_17: cast zx s_7_16 -> i
        let s_7_17: i128 = (i128::try_from(s_7_16).unwrap());
        // D s_7_18: cast zx s_7_14 -> i
        let s_7_18: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_19: call Elem_read(s_7_15, s_7_17, s_7_18)
        let s_7_19: Bits = Elem_read(state, tracer, s_7_15, s_7_17, s_7_18);
        // D s_7_20: read-var ebytes:i64
        let s_7_20: i64 = fn_state.ebytes;
        // D s_7_21: cast zx s_7_20 -> i
        let s_7_21: i128 = (i128::try_from(s_7_20).unwrap());
        // D s_7_22: read-var address:u32
        let s_7_22: u32 = fn_state.address;
        // D s_7_23: call MemU_set(s_7_22, s_7_21, s_7_19)
        let s_7_23: () = MemU_set(state, tracer, s_7_22, s_7_21, s_7_19);
        // N s_7_24: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var address:u32
        let s_8_0: u32 = fn_state.address;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 32u16);
        // D s_8_2: read-var ebytes:i64
        let s_8_2: i64 = fn_state.ebytes;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: cast cvt s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 128);
        // D s_8_5: add s_8_1 s_8_4
        let s_8_5: Bits = (s_8_1 + s_8_4);
        // D s_8_6: cast reint s_8_5 -> u32
        let s_8_6: u32 = (s_8_5.value() as u32);
        // D s_8_7: read-var r:i64
        let s_8_7: i64 = fn_state.r;
        // D s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_9: read-var d2:i
        let s_8_9: i128 = fn_state.d2;
        // D s_8_10: add s_8_9 s_8_8
        let s_8_10: i128 = (s_8_9 + s_8_8);
        // D s_8_11: call D_read(s_8_10)
        let s_8_11: u64 = D_read(state, tracer, s_8_10);
        // C s_8_12: const #8s : i
        let s_8_12: i128 = 8;
        // D s_8_13: read-var ebytes:i64
        let s_8_13: i64 = fn_state.ebytes;
        // D s_8_14: cast zx s_8_13 -> i
        let s_8_14: i128 = (i128::try_from(s_8_13).unwrap());
        // D s_8_15: mul s_8_12 s_8_14
        let s_8_15: i128 = ((s_8_12) * (s_8_14));
        // D s_8_16: cast reint s_8_15 -> i64
        let s_8_16: i64 = (s_8_15 as i64);
        // D s_8_17: cast zx s_8_16 -> i
        let s_8_17: i128 = (i128::try_from(s_8_16).unwrap());
        // D s_8_18: cast reint s_8_17 -> i64
        let s_8_18: i64 = (s_8_17 as i64);
        // D s_8_19: cast zx s_8_11 -> bv
        let s_8_19: Bits = Bits::new(s_8_11 as u128, 64u16);
        // D s_8_20: read-var e:i64
        let s_8_20: i64 = fn_state.e;
        // D s_8_21: cast zx s_8_20 -> i
        let s_8_21: i128 = (i128::try_from(s_8_20).unwrap());
        // D s_8_22: cast zx s_8_18 -> i
        let s_8_22: i128 = (i128::try_from(s_8_18).unwrap());
        // D s_8_23: call Elem_read(s_8_19, s_8_21, s_8_22)
        let s_8_23: Bits = Elem_read(state, tracer, s_8_19, s_8_21, s_8_22);
        // D s_8_24: read-var ebytes:i64
        let s_8_24: i64 = fn_state.ebytes;
        // D s_8_25: cast zx s_8_24 -> i
        let s_8_25: i128 = (i128::try_from(s_8_24).unwrap());
        // D s_8_26: call MemU_set(s_8_6, s_8_25, s_8_23)
        let s_8_26: () = MemU_set(state, tracer, s_8_6, s_8_25, s_8_23);
        // N s_8_27: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #2s : i
        let s_9_0: i128 = 2;
        // D s_9_1: read-var ebytes:i64
        let s_9_1: i64 = fn_state.ebytes;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: mul s_9_0 s_9_2
        let s_9_3: i128 = ((s_9_0) * (s_9_2));
        // D s_9_4: cast reint s_9_3 -> i64
        let s_9_4: i64 = (s_9_3 as i64);
        // D s_9_5: read-var address:u32
        let s_9_5: u32 = fn_state.address;
        // D s_9_6: cast zx s_9_5 -> bv
        let s_9_6: Bits = Bits::new(s_9_5 as u128, 32u16);
        // D s_9_7: cast zx s_9_4 -> i
        let s_9_7: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_8: cast cvt s_9_7 -> bv
        let s_9_8: Bits = Bits::new(s_9_7 as u128, 128);
        // D s_9_9: add s_9_6 s_9_8
        let s_9_9: Bits = (s_9_6 + s_9_8);
        // D s_9_10: cast reint s_9_9 -> u32
        let s_9_10: u32 = (s_9_9.value() as u32);
        // D s_9_11: write-var address <= s_9_10
        fn_state.address = s_9_10;
        // D s_9_12: read-var e:i64
        let s_9_12: i64 = fn_state.e;
        // C s_9_13: const #1s : i64
        let s_9_13: i64 = 1;
        // D s_9_14: add s_9_12 s_9_13
        let s_9_14: i64 = (s_9_12 + s_9_13);
        // D s_9_15: write-var e <= s_9_14
        fn_state.e = s_9_14;
        // N s_9_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var r:i64
        let s_10_0: i64 = fn_state.r;
        // C s_10_1: const #1s : i64
        let s_10_1: i64 = 1;
        // D s_10_2: add s_10_0 s_10_1
        let s_10_2: i64 = (s_10_0 + s_10_1);
        // D s_10_3: write-var r <= s_10_2
        fn_state.r = s_10_2;
        // N s_10_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var wback:u8
        let s_11_0: bool = fn_state.wback;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var register_index:u8
        let s_13_0: bool = fn_state.register_index;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var n:i64
        let s_14_0: i64 = fn_state.n;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call R_read(s_14_1)
        let s_14_2: u32 = R_read(state, tracer, s_14_1);
        // C s_14_3: const #16s : i
        let s_14_3: i128 = 16;
        // D s_14_4: read-var pairs:i64
        let s_14_4: i64 = fn_state.pairs;
        // D s_14_5: cast zx s_14_4 -> i
        let s_14_5: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_6: mul s_14_3 s_14_5
        let s_14_6: i128 = ((s_14_3) * (s_14_5));
        // D s_14_7: cast reint s_14_6 -> i64
        let s_14_7: i64 = (s_14_6 as i64);
        // D s_14_8: cast zx s_14_2 -> bv
        let s_14_8: Bits = Bits::new(s_14_2 as u128, 32u16);
        // D s_14_9: cast zx s_14_7 -> i
        let s_14_9: i128 = (i128::try_from(s_14_7).unwrap());
        // D s_14_10: cast cvt s_14_9 -> bv
        let s_14_10: Bits = Bits::new(s_14_9 as u128, 128);
        // D s_14_11: add s_14_8 s_14_10
        let s_14_11: Bits = (s_14_8 + s_14_10);
        // D s_14_12: cast reint s_14_11 -> u32
        let s_14_12: u32 = (s_14_11.value() as u32);
        // D s_14_13: read-var n:i64
        let s_14_13: i64 = fn_state.n;
        // D s_14_14: cast zx s_14_13 -> i
        let s_14_14: i128 = (i128::try_from(s_14_13).unwrap());
        // D s_14_15: call R_set(s_14_14, s_14_12)
        let s_14_15: () = R_set(state, tracer, s_14_14, s_14_12);
        // N s_14_16: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var n:i64
        let s_15_0: i64 = fn_state.n;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call R_read(s_15_1)
        let s_15_2: u32 = R_read(state, tracer, s_15_1);
        // D s_15_3: read-var m:i64
        let s_15_3: i64 = fn_state.m;
        // D s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_5: call R_read(s_15_4)
        let s_15_5: u32 = R_read(state, tracer, s_15_4);
        // D s_15_6: cast zx s_15_2 -> bv
        let s_15_6: Bits = Bits::new(s_15_2 as u128, 32u16);
        // D s_15_7: cast zx s_15_5 -> bv
        let s_15_7: Bits = Bits::new(s_15_5 as u128, 32u16);
        // D s_15_8: add s_15_6 s_15_7
        let s_15_8: Bits = (s_15_6 + s_15_7);
        // D s_15_9: cast reint s_15_8 -> u32
        let s_15_9: u32 = (s_15_8.value() as u32);
        // D s_15_10: read-var n:i64
        let s_15_10: i64 = fn_state.n;
        // D s_15_11: cast zx s_15_10 -> i
        let s_15_11: i128 = (i128::try_from(s_15_10).unwrap());
        // D s_15_12: call R_set(s_15_11, s_15_9)
        let s_15_12: () = R_set(state, tracer, s_15_11, s_15_9);
        // N s_15_13: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var accdesc:struct
        let s_16_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_16_1: call AlignmentFault(s_16_0)
        let s_16_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_16_0);
        // D s_16_2: read-var address:u32
        let s_16_2: u32 = fn_state.address;
        // D s_16_3: call AArch32_Abort(s_16_2, s_16_1)
        let s_16_3: () = AArch32_Abort(state, tracer, s_16_2, s_16_1);
        // N s_16_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
