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
use AArch32_Abort::*;
use BigEndian::*;
use IsAligned__1::*;
use MemU_read::*;
use D_read::*;
use neq_int::*;
use AlignmentEnforced::*;
use Elem_set::*;
use R_read::*;
use CheckAdvSIMDEnabled::*;
use R_set::*;
use D_set::*;
use CreateAccDescASIMD::*;
use AlignmentFault::*;
use common::*;
pub fn execute_aarch32_instrs_VLD1_m_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    alignment: i64,
    d__arg: i64,
    ebytes: i64,
    elements: i128,
    m: i64,
    n: i64,
    register_index: bool,
    regs: i64,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_895272: Bits,
        r: i64,
        e: i64,
        address: u32,
        gs_895267: Bits,
        gs_309494: i64,
        gs_309500: i64,
        gs_309507: bool,
        data: Bits,
        gs_895262: Bits,
        d: i128,
        accdesc: ProductType9878976b5bcce9c9,
        gs_895275: Bits,
        alignment: i64,
        d__arg: i64,
        ebytes: i64,
        elements: i128,
        m: i64,
        n: i64,
        register_index: bool,
        regs: i64,
        wback: bool,
    }
    let fn_state = FunctionState {
        alignment,
        d__arg,
        ebytes,
        elements,
        m,
        n,
        register_index,
        regs,
        wback,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var d__arg:i64
        let s_0_0: i64 = fn_state.d__arg;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: write-var d <= s_0_1
        fn_state.d = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call CheckAdvSIMDEnabled(s_0_3)
        let s_0_4: () = CheckAdvSIMDEnabled(state, tracer, s_0_3);
        // N s_0_5: jump b1
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
        // C s_1_4: const #0u : u32
        let s_1_4: u32 = 0;
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
        // N s_1_15: branch s_1_14 b30 b2
        if s_1_14 {
            return block_30(state, tracer, fn_state);
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
        // D s_3_2: read-var regs:i64
        let s_3_2: i64 = fn_state.regs;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#309494 <= s_3_5
        fn_state.gs_309494 = s_3_5;
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
        // D s_4_1: read-var gs#309494:i64
        let s_4_1: i64 = fn_state.gs_309494;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b25 b5
        if s_4_2 {
            return block_25(state, tracer, fn_state);
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
        // D s_5_5: write-var gs#309500 <= s_5_4
        fn_state.gs_309500 = s_5_4;
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
        // D s_6_1: read-var gs#309500:i64
        let s_6_1: i64 = fn_state.gs_309500;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b24 b7
        if s_6_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #8s : i
        let s_7_0: i128 = 8;
        // D s_7_1: read-var ebytes:i64
        let s_7_1: i64 = fn_state.ebytes;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: call neq_int(s_7_2, s_7_0)
        let s_7_3: bool = neq_int(state, tracer, s_7_2, s_7_0);
        // N s_7_4: branch s_7_3 b22 b8
        if s_7_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
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
        // D s_8_4: call IsAligned__1(s_8_1, s_8_3)
        let s_8_4: bool = IsAligned__1(state, tracer, s_8_1, s_8_3);
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b21 b9
        if s_8_5 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#309507 <= s_9_0
        fn_state.gs_309507 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#309507:u8
        let s_10_0: bool = fn_state.gs_309507;
        // N s_10_1: branch s_10_0 b20 b11
        if s_10_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2u : u32
        let s_12_0: u32 = 2;
        // S s_12_1: call BigEndian(s_12_0)
        let s_12_1: bool = BigEndian(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b17 b13
        if s_12_1 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #4s : i64
        let s_13_0: i64 = 4;
        // D s_13_1: read-var address:u32
        let s_13_1: u32 = fn_state.address;
        // D s_13_2: call MemU_read(s_13_1, s_13_0)
        let s_13_2: Bits = MemU_read(state, tracer, s_13_1, s_13_0);
        // D s_13_3: write-var gs#895262 <= s_13_2
        fn_state.gs_895262 = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#895262:bv
        let s_14_0: Bits = fn_state.gs_895262;
        // D s_14_1: cast reint s_14_0 -> u32
        let s_14_1: u32 = (s_14_0.value() as u32);
        // C s_14_2: const #0s : i
        let s_14_2: i128 = 0;
        // D s_14_3: cast zx s_14_1 -> bv
        let s_14_3: Bits = Bits::new(s_14_1 as u128, 32u16);
        // D s_14_4: read-var data:bv
        let s_14_4: Bits = fn_state.data;
        // C s_14_5: const #31s : i
        let s_14_5: i128 = 31;
        // C s_14_6: const #1u : u64
        let s_14_6: u64 = 1;
        // C s_14_7: cast zx s_14_6 -> bv
        let s_14_7: Bits = Bits::new(s_14_6 as u128, 64u16);
        // C s_14_8: lsl s_14_7 s_14_5
        let s_14_8: Bits = s_14_7 << s_14_5;
        // C s_14_9: sub s_14_8 s_14_7
        let s_14_9: Bits = ((s_14_8) - (s_14_7));
        // D s_14_10: and s_14_3 s_14_9
        let s_14_10: Bits = ((s_14_3) & (s_14_9));
        // D s_14_11: lsl s_14_10 s_14_2
        let s_14_11: Bits = s_14_10 << s_14_2;
        // C s_14_12: lsl s_14_9 s_14_2
        let s_14_12: Bits = s_14_9 << s_14_2;
        // C s_14_13: cmpl s_14_12
        let s_14_13: Bits = !s_14_12;
        // D s_14_14: and s_14_4 s_14_13
        let s_14_14: Bits = ((s_14_4) & (s_14_13));
        // D s_14_15: or s_14_14 s_14_11
        let s_14_15: Bits = ((s_14_14) | (s_14_11));
        // D s_14_16: write-var data <= s_14_15
        fn_state.data = s_14_15;
        // C s_14_17: const #4s : i
        let s_14_17: i128 = 4;
        // D s_14_18: read-var address:u32
        let s_14_18: u32 = fn_state.address;
        // D s_14_19: cast zx s_14_18 -> bv
        let s_14_19: Bits = Bits::new(s_14_18 as u128, 32u16);
        // C s_14_20: cast cvt s_14_17 -> bv
        let s_14_20: Bits = Bits::new(s_14_17 as u128, 128);
        // D s_14_21: add s_14_19 s_14_20
        let s_14_21: Bits = (s_14_19 + s_14_20);
        // D s_14_22: cast reint s_14_21 -> u32
        let s_14_22: u32 = (s_14_21.value() as u32);
        // C s_14_23: const #4s : i64
        let s_14_23: i64 = 4;
        // D s_14_24: call MemU_read(s_14_22, s_14_23)
        let s_14_24: Bits = MemU_read(state, tracer, s_14_22, s_14_23);
        // D s_14_25: write-var gs#895267 <= s_14_24
        fn_state.gs_895267 = s_14_24;
        // N s_14_26: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#895267:bv
        let s_15_0: Bits = fn_state.gs_895267;
        // D s_15_1: cast reint s_15_0 -> u32
        let s_15_1: u32 = (s_15_0.value() as u32);
        // C s_15_2: const #32s : i
        let s_15_2: i128 = 32;
        // D s_15_3: cast zx s_15_1 -> bv
        let s_15_3: Bits = Bits::new(s_15_1 as u128, 32u16);
        // D s_15_4: read-var data:bv
        let s_15_4: Bits = fn_state.data;
        // C s_15_5: const #31s : i
        let s_15_5: i128 = 31;
        // C s_15_6: const #1u : u64
        let s_15_6: u64 = 1;
        // C s_15_7: cast zx s_15_6 -> bv
        let s_15_7: Bits = Bits::new(s_15_6 as u128, 64u16);
        // C s_15_8: lsl s_15_7 s_15_5
        let s_15_8: Bits = s_15_7 << s_15_5;
        // C s_15_9: sub s_15_8 s_15_7
        let s_15_9: Bits = ((s_15_8) - (s_15_7));
        // D s_15_10: and s_15_3 s_15_9
        let s_15_10: Bits = ((s_15_3) & (s_15_9));
        // D s_15_11: lsl s_15_10 s_15_2
        let s_15_11: Bits = s_15_10 << s_15_2;
        // C s_15_12: lsl s_15_9 s_15_2
        let s_15_12: Bits = s_15_9 << s_15_2;
        // C s_15_13: cmpl s_15_12
        let s_15_13: Bits = !s_15_12;
        // D s_15_14: and s_15_4 s_15_13
        let s_15_14: Bits = ((s_15_4) & (s_15_13));
        // D s_15_15: or s_15_14 s_15_11
        let s_15_15: Bits = ((s_15_14) | (s_15_11));
        // D s_15_16: write-var data <= s_15_15
        fn_state.data = s_15_15;
        // N s_15_17: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var r:i64
        let s_16_0: i64 = fn_state.r;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: read-var d:i
        let s_16_2: i128 = fn_state.d;
        // D s_16_3: add s_16_2 s_16_1
        let s_16_3: i128 = (s_16_2 + s_16_1);
        // D s_16_4: read-var r:i64
        let s_16_4: i64 = fn_state.r;
        // D s_16_5: cast zx s_16_4 -> i
        let s_16_5: i128 = (i128::try_from(s_16_4).unwrap());
        // D s_16_6: read-var d:i
        let s_16_6: i128 = fn_state.d;
        // D s_16_7: add s_16_6 s_16_5
        let s_16_7: i128 = (s_16_6 + s_16_5);
        // D s_16_8: call D_read(s_16_7)
        let s_16_8: u64 = D_read(state, tracer, s_16_7);
        // C s_16_9: const #8s : i
        let s_16_9: i128 = 8;
        // D s_16_10: read-var ebytes:i64
        let s_16_10: i64 = fn_state.ebytes;
        // D s_16_11: cast zx s_16_10 -> i
        let s_16_11: i128 = (i128::try_from(s_16_10).unwrap());
        // D s_16_12: mul s_16_9 s_16_11
        let s_16_12: i128 = ((s_16_9) * (s_16_11));
        // D s_16_13: cast reint s_16_12 -> i64
        let s_16_13: i64 = (s_16_12 as i64);
        // D s_16_14: cast zx s_16_13 -> i
        let s_16_14: i128 = (i128::try_from(s_16_13).unwrap());
        // D s_16_15: cast reint s_16_14 -> i64
        let s_16_15: i64 = (s_16_14 as i64);
        // D s_16_16: cast zx s_16_8 -> bv
        let s_16_16: Bits = Bits::new(s_16_8 as u128, 64u16);
        // D s_16_17: read-var e:i64
        let s_16_17: i64 = fn_state.e;
        // D s_16_18: cast zx s_16_17 -> i
        let s_16_18: i128 = (i128::try_from(s_16_17).unwrap());
        // D s_16_19: cast zx s_16_15 -> i
        let s_16_19: i128 = (i128::try_from(s_16_15).unwrap());
        // D s_16_20: read-var data:bv
        let s_16_20: Bits = fn_state.data;
        // D s_16_21: call Elem_set(s_16_16, s_16_18, s_16_19, s_16_20)
        let s_16_21: Bits = Elem_set(state, tracer, s_16_16, s_16_18, s_16_19, s_16_20);
        // D s_16_22: cast reint s_16_21 -> u64
        let s_16_22: u64 = (s_16_21.value() as u64);
        // D s_16_23: call D_set(s_16_3, s_16_22)
        let s_16_23: () = D_set(state, tracer, s_16_3, s_16_22);
        // D s_16_24: read-var address:u32
        let s_16_24: u32 = fn_state.address;
        // D s_16_25: cast zx s_16_24 -> bv
        let s_16_25: Bits = Bits::new(s_16_24 as u128, 32u16);
        // D s_16_26: read-var ebytes:i64
        let s_16_26: i64 = fn_state.ebytes;
        // D s_16_27: cast zx s_16_26 -> i
        let s_16_27: i128 = (i128::try_from(s_16_26).unwrap());
        // D s_16_28: cast cvt s_16_27 -> bv
        let s_16_28: Bits = Bits::new(s_16_27 as u128, 128);
        // D s_16_29: add s_16_25 s_16_28
        let s_16_29: Bits = (s_16_25 + s_16_28);
        // D s_16_30: cast reint s_16_29 -> u32
        let s_16_30: u32 = (s_16_29.value() as u32);
        // D s_16_31: write-var address <= s_16_30
        fn_state.address = s_16_30;
        // D s_16_32: read-var e:i64
        let s_16_32: i64 = fn_state.e;
        // C s_16_33: const #1s : i64
        let s_16_33: i64 = 1;
        // D s_16_34: add s_16_32 s_16_33
        let s_16_34: i64 = (s_16_32 + s_16_33);
        // D s_16_35: write-var e <= s_16_34
        fn_state.e = s_16_34;
        // N s_16_36: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #4s : i
        let s_17_0: i128 = 4;
        // D s_17_1: read-var address:u32
        let s_17_1: u32 = fn_state.address;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 32u16);
        // C s_17_3: cast cvt s_17_0 -> bv
        let s_17_3: Bits = Bits::new(s_17_0 as u128, 128);
        // D s_17_4: add s_17_2 s_17_3
        let s_17_4: Bits = (s_17_2 + s_17_3);
        // D s_17_5: cast reint s_17_4 -> u32
        let s_17_5: u32 = (s_17_4.value() as u32);
        // C s_17_6: const #4s : i64
        let s_17_6: i64 = 4;
        // D s_17_7: call MemU_read(s_17_5, s_17_6)
        let s_17_7: Bits = MemU_read(state, tracer, s_17_5, s_17_6);
        // D s_17_8: write-var gs#895272 <= s_17_7
        fn_state.gs_895272 = s_17_7;
        // N s_17_9: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#895272:bv
        let s_18_0: Bits = fn_state.gs_895272;
        // D s_18_1: cast reint s_18_0 -> u32
        let s_18_1: u32 = (s_18_0.value() as u32);
        // C s_18_2: const #0s : i
        let s_18_2: i128 = 0;
        // D s_18_3: cast zx s_18_1 -> bv
        let s_18_3: Bits = Bits::new(s_18_1 as u128, 32u16);
        // D s_18_4: read-var data:bv
        let s_18_4: Bits = fn_state.data;
        // C s_18_5: const #31s : i
        let s_18_5: i128 = 31;
        // C s_18_6: const #1u : u64
        let s_18_6: u64 = 1;
        // C s_18_7: cast zx s_18_6 -> bv
        let s_18_7: Bits = Bits::new(s_18_6 as u128, 64u16);
        // C s_18_8: lsl s_18_7 s_18_5
        let s_18_8: Bits = s_18_7 << s_18_5;
        // C s_18_9: sub s_18_8 s_18_7
        let s_18_9: Bits = ((s_18_8) - (s_18_7));
        // D s_18_10: and s_18_3 s_18_9
        let s_18_10: Bits = ((s_18_3) & (s_18_9));
        // D s_18_11: lsl s_18_10 s_18_2
        let s_18_11: Bits = s_18_10 << s_18_2;
        // C s_18_12: lsl s_18_9 s_18_2
        let s_18_12: Bits = s_18_9 << s_18_2;
        // C s_18_13: cmpl s_18_12
        let s_18_13: Bits = !s_18_12;
        // D s_18_14: and s_18_4 s_18_13
        let s_18_14: Bits = ((s_18_4) & (s_18_13));
        // D s_18_15: or s_18_14 s_18_11
        let s_18_15: Bits = ((s_18_14) | (s_18_11));
        // D s_18_16: write-var data <= s_18_15
        fn_state.data = s_18_15;
        // C s_18_17: const #4s : i64
        let s_18_17: i64 = 4;
        // D s_18_18: read-var address:u32
        let s_18_18: u32 = fn_state.address;
        // D s_18_19: call MemU_read(s_18_18, s_18_17)
        let s_18_19: Bits = MemU_read(state, tracer, s_18_18, s_18_17);
        // D s_18_20: write-var gs#895275 <= s_18_19
        fn_state.gs_895275 = s_18_19;
        // N s_18_21: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#895275:bv
        let s_19_0: Bits = fn_state.gs_895275;
        // D s_19_1: cast reint s_19_0 -> u32
        let s_19_1: u32 = (s_19_0.value() as u32);
        // C s_19_2: const #32s : i
        let s_19_2: i128 = 32;
        // D s_19_3: cast zx s_19_1 -> bv
        let s_19_3: Bits = Bits::new(s_19_1 as u128, 32u16);
        // D s_19_4: read-var data:bv
        let s_19_4: Bits = fn_state.data;
        // C s_19_5: const #31s : i
        let s_19_5: i128 = 31;
        // C s_19_6: const #1u : u64
        let s_19_6: u64 = 1;
        // C s_19_7: cast zx s_19_6 -> bv
        let s_19_7: Bits = Bits::new(s_19_6 as u128, 64u16);
        // C s_19_8: lsl s_19_7 s_19_5
        let s_19_8: Bits = s_19_7 << s_19_5;
        // C s_19_9: sub s_19_8 s_19_7
        let s_19_9: Bits = ((s_19_8) - (s_19_7));
        // D s_19_10: and s_19_3 s_19_9
        let s_19_10: Bits = ((s_19_3) & (s_19_9));
        // D s_19_11: lsl s_19_10 s_19_2
        let s_19_11: Bits = s_19_10 << s_19_2;
        // C s_19_12: lsl s_19_9 s_19_2
        let s_19_12: Bits = s_19_9 << s_19_2;
        // C s_19_13: cmpl s_19_12
        let s_19_13: Bits = !s_19_12;
        // D s_19_14: and s_19_4 s_19_13
        let s_19_14: Bits = ((s_19_4) & (s_19_13));
        // D s_19_15: or s_19_14 s_19_11
        let s_19_15: Bits = ((s_19_14) | (s_19_11));
        // D s_19_16: write-var data <= s_19_15
        fn_state.data = s_19_15;
        // N s_19_17: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var accdesc:struct
        let s_20_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_20_1: call AlignmentFault(s_20_0)
        let s_20_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_20_0);
        // D s_20_2: read-var address:u32
        let s_20_2: u32 = fn_state.address;
        // D s_20_3: call AArch32_Abort(s_20_2, s_20_1)
        let s_20_3: () = AArch32_Abort(state, tracer, s_20_2, s_20_1);
        // N s_20_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call AlignmentEnforced(s_21_0)
        let s_21_1: bool = AlignmentEnforced(state, tracer, s_21_0);
        // D s_21_2: write-var gs#309507 <= s_21_1
        fn_state.gs_309507 = s_21_1;
        // N s_21_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var address:u32
        let s_22_0: u32 = fn_state.address;
        // D s_22_1: read-var ebytes:i64
        let s_22_1: i64 = fn_state.ebytes;
        // D s_22_2: call MemU_read(s_22_0, s_22_1)
        let s_22_2: Bits = MemU_read(state, tracer, s_22_0, s_22_1);
        // D s_22_3: write-var data <= s_22_2
        fn_state.data = s_22_2;
        // N s_22_4: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var r:i64
        let s_24_0: i64 = fn_state.r;
        // C s_24_1: const #1s : i64
        let s_24_1: i64 = 1;
        // D s_24_2: add s_24_0 s_24_1
        let s_24_2: i64 = (s_24_0 + s_24_1);
        // D s_24_3: write-var r <= s_24_2
        fn_state.r = s_24_2;
        // N s_24_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var wback:u8
        let s_25_0: bool = fn_state.wback;
        // N s_25_1: branch s_25_0 b27 b26
        if s_25_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var register_index:u8
        let s_27_0: bool = fn_state.register_index;
        // N s_27_1: branch s_27_0 b29 b28
        if s_27_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var n:i64
        let s_28_0: i64 = fn_state.n;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: call R_read(s_28_1)
        let s_28_2: u32 = R_read(state, tracer, s_28_1);
        // C s_28_3: const #8s : i
        let s_28_3: i128 = 8;
        // D s_28_4: read-var regs:i64
        let s_28_4: i64 = fn_state.regs;
        // D s_28_5: cast zx s_28_4 -> i
        let s_28_5: i128 = (i128::try_from(s_28_4).unwrap());
        // D s_28_6: mul s_28_3 s_28_5
        let s_28_6: i128 = ((s_28_3) * (s_28_5));
        // D s_28_7: cast reint s_28_6 -> i64
        let s_28_7: i64 = (s_28_6 as i64);
        // D s_28_8: cast zx s_28_2 -> bv
        let s_28_8: Bits = Bits::new(s_28_2 as u128, 32u16);
        // D s_28_9: cast zx s_28_7 -> i
        let s_28_9: i128 = (i128::try_from(s_28_7).unwrap());
        // D s_28_10: cast cvt s_28_9 -> bv
        let s_28_10: Bits = Bits::new(s_28_9 as u128, 128);
        // D s_28_11: add s_28_8 s_28_10
        let s_28_11: Bits = (s_28_8 + s_28_10);
        // D s_28_12: cast reint s_28_11 -> u32
        let s_28_12: u32 = (s_28_11.value() as u32);
        // D s_28_13: read-var n:i64
        let s_28_13: i64 = fn_state.n;
        // D s_28_14: cast zx s_28_13 -> i
        let s_28_14: i128 = (i128::try_from(s_28_13).unwrap());
        // D s_28_15: call R_set(s_28_14, s_28_12)
        let s_28_15: () = R_set(state, tracer, s_28_14, s_28_12);
        // N s_28_16: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var n:i64
        let s_29_0: i64 = fn_state.n;
        // D s_29_1: cast zx s_29_0 -> i
        let s_29_1: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_2: call R_read(s_29_1)
        let s_29_2: u32 = R_read(state, tracer, s_29_1);
        // D s_29_3: read-var m:i64
        let s_29_3: i64 = fn_state.m;
        // D s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // D s_29_5: call R_read(s_29_4)
        let s_29_5: u32 = R_read(state, tracer, s_29_4);
        // D s_29_6: cast zx s_29_2 -> bv
        let s_29_6: Bits = Bits::new(s_29_2 as u128, 32u16);
        // D s_29_7: cast zx s_29_5 -> bv
        let s_29_7: Bits = Bits::new(s_29_5 as u128, 32u16);
        // D s_29_8: add s_29_6 s_29_7
        let s_29_8: Bits = (s_29_6 + s_29_7);
        // D s_29_9: cast reint s_29_8 -> u32
        let s_29_9: u32 = (s_29_8.value() as u32);
        // D s_29_10: read-var n:i64
        let s_29_10: i64 = fn_state.n;
        // D s_29_11: cast zx s_29_10 -> i
        let s_29_11: i128 = (i128::try_from(s_29_10).unwrap());
        // D s_29_12: call R_set(s_29_11, s_29_9)
        let s_29_12: () = R_set(state, tracer, s_29_11, s_29_9);
        // N s_29_13: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var accdesc:struct
        let s_30_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_30_1: call AlignmentFault(s_30_0)
        let s_30_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_30_0);
        // D s_30_2: read-var address:u32
        let s_30_2: u32 = fn_state.address;
        // D s_30_3: call AArch32_Abort(s_30_2, s_30_1)
        let s_30_3: () = AArch32_Abort(state, tracer, s_30_2, s_30_1);
        // N s_30_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
