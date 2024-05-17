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
use V_read::*;
use SP_set::*;
use fmod_int::*;
use X_read::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use neq_int::*;
use X_set::*;
use Elem_set::*;
use V_set::*;
use Mem_read::*;
use Mem_set::*;
use SP_read::*;
use SPESampleSIMDFPLoadStore::*;
use CreateAccDescASIMD::*;
use CheckSPAlignment::*;
use common::*;
pub fn execute_aarch64_instrs_memory_vector_multiple_no_wb<T: Tracer>(
    state: &mut State,
    tracer: &T,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i128,
    memop: u32,
    n: i64,
    nontemporal: bool,
    rpt: i64,
    selem: i64,
    t: i64,
    tagchecked: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        tt: i128,
        datasizeshadow_1539: i64,
        ebytes: i64,
        e: i64,
        ga_258087: i64,
        address: u64,
        esizeshadow_1538: i64,
        offs: u64,
        rval: Bits,
        gs_156737: i64,
        s: i64,
        gs_156745: i64,
        gs_156731: i64,
        accdesc: ProductType9878976b5bcce9c9,
        ttshadow_1541: i128,
        ga_258088: Bits,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i128,
        memop: u32,
        n: i64,
        nontemporal: bool,
        rpt: i64,
        selem: i64,
        t: i64,
        tagchecked: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        datasize,
        elements,
        esize,
        m,
        memop,
        n,
        nontemporal,
        rpt,
        selem,
        t,
        tagchecked,
        wback,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#1538 <= s_0_2
        fn_state.esizeshadow_1538 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1539 <= s_0_6
        fn_state.datasizeshadow_1539 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPAdvSIMDEnabled64(s_0_8)
        let s_0_9: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #8s : i
        let s_1_0: i128 = 8;
        // D s_1_1: read-var esizeshadow#1538:i64
        let s_1_1: i64 = fn_state.esizeshadow_1538;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: div s_1_2 s_1_0
        let s_1_3: i128 = ((s_1_2) / (s_1_0));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var ebytes <= s_1_4
        fn_state.ebytes = s_1_4;
        // D s_1_6: read-var memop:u32
        let s_1_6: u32 = fn_state.memop;
        // D s_1_7: read-var nontemporal:u8
        let s_1_7: bool = fn_state.nontemporal;
        // D s_1_8: read-var tagchecked:u8
        let s_1_8: bool = fn_state.tagchecked;
        // D s_1_9: call CreateAccDescASIMD(s_1_6, s_1_7, s_1_8)
        let s_1_9: ProductType9878976b5bcce9c9 = CreateAccDescASIMD(
            state,
            tracer,
            s_1_6,
            s_1_7,
            s_1_8,
        );
        // D s_1_10: write-var accdesc <= s_1_9
        fn_state.accdesc = s_1_9;
        // C s_1_11: const #31s : i
        let s_1_11: i128 = 31;
        // D s_1_12: read-var n:i64
        let s_1_12: i64 = fn_state.n;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cmp-eq s_1_13 s_1_11
        let s_1_14: bool = ((s_1_13) == (s_1_11));
        // N s_1_15: branch s_1_14 b27 b2
        if s_1_14 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: write-var address <= s_2_4
        fn_state.address = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u64
        let s_3_0: u64 = 0;
        // D s_3_1: write-var offs <= s_3_0
        fn_state.offs = s_3_0;
        // C s_3_2: const #0s : i64
        let s_3_2: i64 = 0;
        // C s_3_3: const #1s : i
        let s_3_3: i128 = 1;
        // D s_3_4: read-var rpt:i64
        let s_3_4: i64 = fn_state.rpt;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: sub s_3_5 s_3_3
        let s_3_6: i128 = ((s_3_5) - (s_3_3));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: write-var gs#156731 <= s_3_7
        fn_state.gs_156731 = s_3_7;
        // D s_3_9: write-var r <= s_3_2
        fn_state.r = s_3_2;
        // N s_3_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:i64
        let s_4_0: i64 = fn_state.r;
        // D s_4_1: read-var gs#156731:i64
        let s_4_1: i64 = fn_state.gs_156731;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b16 b5
        if s_4_2 {
            return block_16(state, tracer, fn_state);
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
        // D s_5_5: write-var gs#156737 <= s_5_4
        fn_state.gs_156737 = s_5_4;
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
        // D s_6_1: read-var gs#156737:i64
        let s_6_1: i64 = fn_state.gs_156737;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b15 b7
        if s_6_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var t:i64
        let s_7_0: i64 = fn_state.t;
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
        // C s_7_6: const #32s : i
        let s_7_6: i128 = 32;
        // D s_7_7: cast zx s_7_5 -> i
        let s_7_7: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_8: mod s_7_7 s_7_6
        let s_7_8: i128 = ((s_7_7) % (s_7_6));
        // D s_7_9: write-var tt <= s_7_8
        fn_state.tt = s_7_8;
        // C s_7_10: const #0s : i64
        let s_7_10: i64 = 0;
        // C s_7_11: const #1s : i
        let s_7_11: i128 = 1;
        // D s_7_12: read-var selem:i64
        let s_7_12: i64 = fn_state.selem;
        // D s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_14: sub s_7_13 s_7_11
        let s_7_14: i128 = ((s_7_13) - (s_7_11));
        // D s_7_15: cast reint s_7_14 -> i64
        let s_7_15: i64 = (s_7_14 as i64);
        // D s_7_16: write-var gs#156745 <= s_7_15
        fn_state.gs_156745 = s_7_15;
        // D s_7_17: write-var s <= s_7_10
        fn_state.s = s_7_10;
        // N s_7_18: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var s:i64
        let s_8_0: i64 = fn_state.s;
        // D s_8_1: read-var gs#156745:i64
        let s_8_1: i64 = fn_state.gs_156745;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b14 b9
        if s_8_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var tt:i
        let s_9_0: i128 = fn_state.tt;
        // D s_9_1: read-var datasizeshadow#1539:i64
        let s_9_1: i64 = fn_state.datasizeshadow_1539;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: call V_read(s_9_0, s_9_3)
        let s_9_4: Bits = V_read(state, tracer, s_9_0, s_9_3);
        // D s_9_5: write-var rval <= s_9_4
        fn_state.rval = s_9_4;
        // D s_9_6: read-var tt:i
        let s_9_6: i128 = fn_state.tt;
        // D s_9_7: write-var ttshadow#1541 <= s_9_6
        fn_state.ttshadow_1541 = s_9_6;
        // D s_9_8: read-var memop:u32
        let s_9_8: u32 = fn_state.memop;
        // C s_9_9: const #0u : u32
        let s_9_9: u32 = 0;
        // D s_9_10: cmp-eq s_9_8 s_9_9
        let s_9_10: bool = ((s_9_8) == (s_9_9));
        // N s_9_11: branch s_9_10 b12 b10
        if s_9_10 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var address:u64
        let s_10_0: u64 = fn_state.address;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 64u16);
        // D s_10_2: read-var offs:u64
        let s_10_2: u64 = fn_state.offs;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 64u16);
        // D s_10_4: add s_10_1 s_10_3
        let s_10_4: Bits = (s_10_1 + s_10_3);
        // D s_10_5: cast reint s_10_4 -> u64
        let s_10_5: u64 = (s_10_4.value() as u64);
        // D s_10_6: read-var esizeshadow#1538:i64
        let s_10_6: i64 = fn_state.esizeshadow_1538;
        // D s_10_7: cast zx s_10_6 -> i
        let s_10_7: i128 = (i128::try_from(s_10_6).unwrap());
        // D s_10_8: cast reint s_10_7 -> i64
        let s_10_8: i64 = (s_10_7 as i64);
        // D s_10_9: read-var e:i64
        let s_10_9: i64 = fn_state.e;
        // D s_10_10: cast zx s_10_9 -> i
        let s_10_10: i128 = (i128::try_from(s_10_9).unwrap());
        // D s_10_11: cast zx s_10_8 -> i
        let s_10_11: i128 = (i128::try_from(s_10_8).unwrap());
        // D s_10_12: read-var rval:bv
        let s_10_12: Bits = fn_state.rval;
        // D s_10_13: call Elem_read(s_10_12, s_10_10, s_10_11)
        let s_10_13: Bits = Elem_read(state, tracer, s_10_12, s_10_10, s_10_11);
        // D s_10_14: read-var ebytes:i64
        let s_10_14: i64 = fn_state.ebytes;
        // D s_10_15: cast zx s_10_14 -> i
        let s_10_15: i128 = (i128::try_from(s_10_14).unwrap());
        // D s_10_16: read-var accdesc:struct
        let s_10_16: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_10_17: call Mem_set(s_10_5, s_10_15, s_10_16, s_10_13)
        let s_10_17: () = Mem_set(state, tracer, s_10_5, s_10_15, s_10_16, s_10_13);
        // N s_10_18: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var offs:u64
        let s_11_0: u64 = fn_state.offs;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 64u16);
        // D s_11_2: read-var ebytes:i64
        let s_11_2: i64 = fn_state.ebytes;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: cast cvt s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 128);
        // D s_11_5: add s_11_1 s_11_4
        let s_11_5: Bits = (s_11_1 + s_11_4);
        // D s_11_6: cast reint s_11_5 -> u64
        let s_11_6: u64 = (s_11_5.value() as u64);
        // D s_11_7: write-var offs <= s_11_6
        fn_state.offs = s_11_6;
        // C s_11_8: const #1s : i
        let s_11_8: i128 = 1;
        // D s_11_9: read-var tt:i
        let s_11_9: i128 = fn_state.tt;
        // D s_11_10: add s_11_9 s_11_8
        let s_11_10: i128 = (s_11_9 + s_11_8);
        // C s_11_11: const #32s : i
        let s_11_11: i128 = 32;
        // D s_11_12: call fmod_int(s_11_10, s_11_11)
        let s_11_12: i128 = fmod_int(state, tracer, s_11_10, s_11_11);
        // D s_11_13: write-var tt <= s_11_12
        fn_state.tt = s_11_12;
        // D s_11_14: read-var s:i64
        let s_11_14: i64 = fn_state.s;
        // C s_11_15: const #1s : i64
        let s_11_15: i64 = 1;
        // D s_11_16: add s_11_14 s_11_15
        let s_11_16: i64 = (s_11_14 + s_11_15);
        // D s_11_17: write-var s <= s_11_16
        fn_state.s = s_11_16;
        // N s_11_18: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esizeshadow#1538:i64
        let s_12_0: i64 = fn_state.esizeshadow_1538;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: write-var ga#258087 <= s_12_2
        fn_state.ga_258087 = s_12_2;
        // D s_12_4: read-var address:u64
        let s_12_4: u64 = fn_state.address;
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 64u16);
        // D s_12_6: read-var offs:u64
        let s_12_6: u64 = fn_state.offs;
        // D s_12_7: cast zx s_12_6 -> bv
        let s_12_7: Bits = Bits::new(s_12_6 as u128, 64u16);
        // D s_12_8: add s_12_5 s_12_7
        let s_12_8: Bits = (s_12_5 + s_12_7);
        // D s_12_9: cast reint s_12_8 -> u64
        let s_12_9: u64 = (s_12_8.value() as u64);
        // D s_12_10: read-var ebytes:i64
        let s_12_10: i64 = fn_state.ebytes;
        // D s_12_11: cast zx s_12_10 -> i
        let s_12_11: i128 = (i128::try_from(s_12_10).unwrap());
        // D s_12_12: read-var accdesc:struct
        let s_12_12: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_12_13: call Mem_read(s_12_9, s_12_11, s_12_12)
        let s_12_13: Bits = Mem_read(state, tracer, s_12_9, s_12_11, s_12_12);
        // D s_12_14: write-var ga#258088 <= s_12_13
        fn_state.ga_258088 = s_12_13;
        // N s_12_15: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var e:i64
        let s_13_0: i64 = fn_state.e;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var ga#258087:i64
        let s_13_2: i64 = fn_state.ga_258087;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: read-var rval:bv
        let s_13_4: Bits = fn_state.rval;
        // D s_13_5: read-var ga#258088:bv
        let s_13_5: Bits = fn_state.ga_258088;
        // D s_13_6: call Elem_set(s_13_4, s_13_1, s_13_3, s_13_5)
        let s_13_6: Bits = Elem_set(state, tracer, s_13_4, s_13_1, s_13_3, s_13_5);
        // D s_13_7: write-var rval <= s_13_6
        fn_state.rval = s_13_6;
        // D s_13_8: read-var datasizeshadow#1539:i64
        let s_13_8: i64 = fn_state.datasizeshadow_1539;
        // D s_13_9: cast zx s_13_8 -> i
        let s_13_9: i128 = (i128::try_from(s_13_8).unwrap());
        // D s_13_10: cast reint s_13_9 -> i64
        let s_13_10: i64 = (s_13_9 as i64);
        // D s_13_11: read-var ttshadow#1541:i
        let s_13_11: i128 = fn_state.ttshadow_1541;
        // D s_13_12: read-var rval:bv
        let s_13_12: Bits = fn_state.rval;
        // D s_13_13: call V_set(s_13_11, s_13_10, s_13_12)
        let s_13_13: () = V_set(state, tracer, s_13_11, s_13_10, s_13_12);
        // N s_13_14: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var e:i64
        let s_14_0: i64 = fn_state.e;
        // C s_14_1: const #1s : i64
        let s_14_1: i64 = 1;
        // D s_14_2: add s_14_0 s_14_1
        let s_14_2: i64 = (s_14_0 + s_14_1);
        // D s_14_3: write-var e <= s_14_2
        fn_state.e = s_14_2;
        // N s_14_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var r:i64
        let s_15_0: i64 = fn_state.r;
        // C s_15_1: const #1s : i64
        let s_15_1: i64 = 1;
        // D s_15_2: add s_15_0 s_15_1
        let s_15_2: i64 = (s_15_0 + s_15_1);
        // D s_15_3: write-var r <= s_15_2
        fn_state.r = s_15_2;
        // N s_15_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #22416u : u32
        let s_16_0: u32 = 22416;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: bool = {
            let value = state.read_register::<bool>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // N s_16_2: branch s_16_1 b26 b17
        if s_16_1 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var wback:u8
        let s_18_0: bool = fn_state.wback;
        // N s_18_1: branch s_18_0 b20 b19
        if s_18_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #31s : i
        let s_20_0: i128 = 31;
        // D s_20_1: read-var m:i
        let s_20_1: i128 = fn_state.m;
        // D s_20_2: call neq_int(s_20_1, s_20_0)
        let s_20_2: bool = neq_int(state, tracer, s_20_1, s_20_0);
        // N s_20_3: branch s_20_2 b25 b21
        if s_20_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #31s : i
        let s_22_0: i128 = 31;
        // D s_22_1: read-var n:i64
        let s_22_1: i64 = fn_state.n;
        // D s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (i128::try_from(s_22_1).unwrap());
        // D s_22_3: cmp-eq s_22_2 s_22_0
        let s_22_3: bool = ((s_22_2) == (s_22_0));
        // N s_22_4: branch s_22_3 b24 b23
        if s_22_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #64s : i64
        let s_23_0: i64 = 64;
        // D s_23_1: read-var address:u64
        let s_23_1: u64 = fn_state.address;
        // D s_23_2: cast zx s_23_1 -> bv
        let s_23_2: Bits = Bits::new(s_23_1 as u128, 64u16);
        // D s_23_3: read-var offs:u64
        let s_23_3: u64 = fn_state.offs;
        // D s_23_4: cast zx s_23_3 -> bv
        let s_23_4: Bits = Bits::new(s_23_3 as u128, 64u16);
        // D s_23_5: add s_23_2 s_23_4
        let s_23_5: Bits = (s_23_2 + s_23_4);
        // D s_23_6: cast reint s_23_5 -> u64
        let s_23_6: u64 = (s_23_5.value() as u64);
        // D s_23_7: read-var n:i64
        let s_23_7: i64 = fn_state.n;
        // D s_23_8: cast zx s_23_7 -> i
        let s_23_8: i128 = (i128::try_from(s_23_7).unwrap());
        // D s_23_9: cast zx s_23_6 -> bv
        let s_23_9: Bits = Bits::new(s_23_6 as u128, 64u16);
        // D s_23_10: call X_set(s_23_8, s_23_0, s_23_9)
        let s_23_10: () = X_set(state, tracer, s_23_8, s_23_0, s_23_9);
        // N s_23_11: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var address:u64
        let s_24_0: u64 = fn_state.address;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 64u16);
        // D s_24_2: read-var offs:u64
        let s_24_2: u64 = fn_state.offs;
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 64u16);
        // D s_24_4: add s_24_1 s_24_3
        let s_24_4: Bits = (s_24_1 + s_24_3);
        // D s_24_5: cast reint s_24_4 -> u64
        let s_24_5: u64 = (s_24_4.value() as u64);
        // D s_24_6: call SP_set(s_24_5)
        let s_24_6: () = SP_set(state, tracer, s_24_5);
        // N s_24_7: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #64s : i64
        let s_25_0: i64 = 64;
        // D s_25_1: read-var m:i
        let s_25_1: i128 = fn_state.m;
        // D s_25_2: call X_read(s_25_1, s_25_0)
        let s_25_2: Bits = X_read(state, tracer, s_25_1, s_25_0);
        // D s_25_3: cast reint s_25_2 -> u64
        let s_25_3: u64 = (s_25_2.value() as u64);
        // D s_25_4: write-var offs <= s_25_3
        fn_state.offs = s_25_3;
        // N s_25_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call SPESampleSIMDFPLoadStore(s_26_0)
        let s_26_1: () = SPESampleSIMDFPLoadStore(state, tracer, s_26_0);
        // N s_26_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call CheckSPAlignment(s_27_0)
        let s_27_1: () = CheckSPAlignment(state, tracer, s_27_0);
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call SP_read(s_28_0)
        let s_28_1: u64 = SP_read(state, tracer, s_28_0);
        // D s_28_2: write-var address <= s_28_1
        fn_state.address = s_28_1;
        // N s_28_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
