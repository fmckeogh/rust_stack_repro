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
use neq_int::*;
use CreateAccDescSVE::*;
use SP_read::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use P_read::*;
use ActivePredicateElement::*;
use ConstrainUnpredictableBool::*;
use X_read::*;
use Elem_read::*;
use CheckSPAlignment::*;
use Z_read::*;
use Mem_set::*;
use common::*;
pub fn execute_ST2H_Z_P_BI_Contiguous<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    n: i64,
    nreg: i64,
    offset: i128,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        values_name: [Bits; 2usize],
        base: u64,
        u_6703: i64,
        mbytes: i64,
        gs_247194: i64,
        VLshadow_5170: i64,
        gs_247211: i64,
        mask: Bits,
        VLshadow_5169: i64,
        accdesc: ProductType9878976b5bcce9c9,
        elements: i64,
        gs_247217: i64,
        gs_247206: bool,
        VL: i64,
        esize: i64,
        g: i64,
        n: i64,
        nreg: i64,
        offset: i128,
        t: i64,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        n,
        nreg,
        offset,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#5169 <= s_0_2
        fn_state.VLshadow_5169 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckSVEEnabled(s_0_4)
        let s_0_5: () = CheckSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#5169:i64
        let s_1_0: i64 = fn_state.VLshadow_5169;
        // D s_1_1: write-var VLshadow#5170 <= s_1_0
        fn_state.VLshadow_5170 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#5170:i64
        let s_1_3: i64 = fn_state.VLshadow_5170;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#5170:i64
        let s_1_7: i64 = fn_state.VLshadow_5170;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esize:i64
        let s_1_9: i64 = fn_state.esize;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var elements <= s_1_12
        fn_state.elements = s_1_12;
        // D s_1_14: cast zx s_1_6 -> i
        let s_1_14: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var g:i64
        let s_1_16: i64 = fn_state.g;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call P_read(s_1_17, s_1_18)
        let s_1_19: Bits = P_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: write-var mask <= s_1_19
        fn_state.mask = s_1_19;
        // C s_1_21: const #8s : i
        let s_1_21: i128 = 8;
        // D s_1_22: read-var esize:i64
        let s_1_22: i64 = fn_state.esize;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: div s_1_23 s_1_21
        let s_1_24: i128 = ((s_1_23) / (s_1_21));
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: write-var mbytes <= s_1_25
        fn_state.mbytes = s_1_25;
        // C s_1_27: const #31s : i
        let s_1_27: i128 = 31;
        // D s_1_28: read-var n:i64
        let s_1_28: i64 = fn_state.n;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: call neq_int(s_1_29, s_1_27)
        let s_1_30: bool = neq_int(state, tracer, s_1_29, s_1_27);
        // C s_1_31: const #1u : u32
        let s_1_31: u32 = 1;
        // C s_1_32: const #0u : u8
        let s_1_32: bool = false;
        // C s_1_33: const #1u : u8
        let s_1_33: bool = true;
        // D s_1_34: call CreateAccDescSVE(s_1_31, s_1_32, s_1_33, s_1_30)
        let s_1_34: ProductType9878976b5bcce9c9 = CreateAccDescSVE(
            state,
            tracer,
            s_1_31,
            s_1_32,
            s_1_33,
            s_1_30,
        );
        // D s_1_35: write-var accdesc <= s_1_34
        fn_state.accdesc = s_1_34;
        // D s_1_36: read-var esize:i64
        let s_1_36: i64 = fn_state.esize;
        // D s_1_37: cast zx s_1_36 -> i
        let s_1_37: i128 = (i128::try_from(s_1_36).unwrap());
        // D s_1_38: read-var mask:bv
        let s_1_38: Bits = fn_state.mask;
        // D s_1_39: call AnyActiveElement(s_1_38, s_1_37)
        let s_1_39: bool = AnyActiveElement(state, tracer, s_1_38, s_1_37);
        // D s_1_40: not s_1_39
        let s_1_40: bool = !s_1_39;
        // N s_1_41: branch s_1_40 b23 b2
        if s_1_40 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #31s : i
        let s_2_0: i128 = 31;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_0
        let s_2_3: bool = ((s_2_2) == (s_2_0));
        // N s_2_4: branch s_2_3 b22 b3
        if s_2_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #31s : i
        let s_4_0: i128 = 31;
        // D s_4_1: read-var n:i64
        let s_4_1: i64 = fn_state.n;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // N s_4_4: branch s_4_3 b21 b5
        if s_4_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: call X_read(s_5_2, s_5_0)
        let s_5_3: Bits = X_read(state, tracer, s_5_2, s_5_0);
        // D s_5_4: cast reint s_5_3 -> u64
        let s_5_4: u64 = (s_5_3.value() as u64);
        // D s_5_5: write-var base <= s_5_4
        fn_state.base = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i64
        let s_7_0: i64 = 0;
        // C s_7_1: const #1s : i
        let s_7_1: i128 = 1;
        // D s_7_2: read-var nreg:i64
        let s_7_2: i64 = fn_state.nreg;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: sub s_7_3 s_7_1
        let s_7_4: i128 = ((s_7_3) - (s_7_1));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: write-var gs#247194 <= s_7_5
        fn_state.gs_247194 = s_7_5;
        // D s_7_7: write-var r <= s_7_0
        fn_state.r = s_7_0;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var r:i64
        let s_8_0: i64 = fn_state.r;
        // D s_8_1: read-var gs#247194:i64
        let s_8_1: i64 = fn_state.gs_247194;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b10 b9
        if s_8_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var t:i64
        let s_9_0: i64 = fn_state.t;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var r:i64
        let s_9_2: i64 = fn_state.r;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: add s_9_1 s_9_3
        let s_9_4: i128 = (s_9_1 + s_9_3);
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // C s_9_6: const #32s : i
        let s_9_6: i128 = 32;
        // D s_9_7: cast zx s_9_5 -> i
        let s_9_7: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_8: mod s_9_7 s_9_6
        let s_9_8: i128 = ((s_9_7) % (s_9_6));
        // D s_9_9: cast reint s_9_8 -> i64
        let s_9_9: i64 = (s_9_8 as i64);
        // D s_9_10: read-var VLshadow#5170:i64
        let s_9_10: i64 = fn_state.VLshadow_5170;
        // D s_9_11: cast zx s_9_10 -> i
        let s_9_11: i128 = (i128::try_from(s_9_10).unwrap());
        // D s_9_12: cast reint s_9_11 -> i64
        let s_9_12: i64 = (s_9_11 as i64);
        // D s_9_13: cast zx s_9_9 -> i
        let s_9_13: i128 = (i128::try_from(s_9_9).unwrap());
        // D s_9_14: cast zx s_9_12 -> i
        let s_9_14: i128 = (i128::try_from(s_9_12).unwrap());
        // D s_9_15: call Z_read(s_9_13, s_9_14)
        let s_9_15: Bits = Z_read(state, tracer, s_9_13, s_9_14);
        // D s_9_16: read-var values_name:[bv; 2]
        let s_9_16: [Bits; 2usize] = fn_state.values_name;
        // D s_9_17: cast cvt s_9_16 -> [bv; 0]
        let s_9_17: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_9_16);
        // D s_9_18: read-var r:i64
        let s_9_18: i64 = fn_state.r;
        // D s_9_19: cast zx s_9_18 -> i
        let s_9_19: i128 = (i128::try_from(s_9_18).unwrap());
        // D s_9_20: mutate-element s_9_17[s_9_19] <= s_9_15
        let s_9_20: alloc::vec::Vec<Bits> = {
            let mut local = s_9_17.clone();
            local[(s_9_19) as usize] = s_9_15;
            local
        };
        // D s_9_21: cast cvt s_9_20 -> [bv; 2]
        let s_9_21: [Bits; 2usize] = {
            let mut buf = [Default::default(); 2usize];
            buf.copy_from_slice(&s_9_20);
            buf
        };
        // D s_9_22: write-var values_name <= s_9_21
        fn_state.values_name = s_9_21;
        // D s_9_23: read-var r:i64
        let s_9_23: i64 = fn_state.r;
        // C s_9_24: const #1s : i64
        let s_9_24: i64 = 1;
        // D s_9_25: add s_9_23 s_9_24
        let s_9_25: i64 = (s_9_23 + s_9_24);
        // D s_9_26: write-var r <= s_9_25
        fn_state.r = s_9_25;
        // N s_9_27: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i64
        let s_10_0: i64 = 0;
        // C s_10_1: const #1s : i
        let s_10_1: i128 = 1;
        // D s_10_2: read-var elements:i64
        let s_10_2: i64 = fn_state.elements;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: sub s_10_3 s_10_1
        let s_10_4: i128 = ((s_10_3) - (s_10_1));
        // D s_10_5: cast reint s_10_4 -> i64
        let s_10_5: i64 = (s_10_4 as i64);
        // D s_10_6: write-var gs#247211 <= s_10_5
        fn_state.gs_247211 = s_10_5;
        // D s_10_7: write-var e <= s_10_0
        fn_state.e = s_10_0;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var e:i64
        let s_11_0: i64 = fn_state.e;
        // D s_11_1: read-var gs#247211:i64
        let s_11_1: i64 = fn_state.gs_247211;
        // D s_11_2: cmp-gt s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) > (s_11_1));
        // N s_11_3: branch s_11_2 b20 b12
        if s_11_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i64
        let s_12_0: i64 = 0;
        // C s_12_1: const #1s : i
        let s_12_1: i128 = 1;
        // D s_12_2: read-var nreg:i64
        let s_12_2: i64 = fn_state.nreg;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: sub s_12_3 s_12_1
        let s_12_4: i128 = ((s_12_3) - (s_12_1));
        // D s_12_5: cast reint s_12_4 -> i64
        let s_12_5: i64 = (s_12_4 as i64);
        // D s_12_6: write-var gs#247217 <= s_12_5
        fn_state.gs_247217 = s_12_5;
        // D s_12_7: write-var u#6703 <= s_12_0
        fn_state.u_6703 = s_12_0;
        // N s_12_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var u#6703:i64
        let s_13_0: i64 = fn_state.u_6703;
        // D s_13_1: read-var gs#247217:i64
        let s_13_1: i64 = fn_state.gs_247217;
        // D s_13_2: cmp-gt s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) > (s_13_1));
        // N s_13_3: branch s_13_2 b19 b14
        if s_13_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var e:i64
        let s_14_0: i64 = fn_state.e;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var esize:i64
        let s_14_2: i64 = fn_state.esize;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: read-var mask:bv
        let s_14_4: Bits = fn_state.mask;
        // D s_14_5: call ActivePredicateElement(s_14_4, s_14_1, s_14_3)
        let s_14_5: bool = ActivePredicateElement(state, tracer, s_14_4, s_14_1, s_14_3);
        // N s_14_6: branch s_14_5 b17 b15
        if s_14_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var u#6703:i64
        let s_16_0: i64 = fn_state.u_6703;
        // C s_16_1: const #1s : i64
        let s_16_1: i64 = 1;
        // D s_16_2: add s_16_0 s_16_1
        let s_16_2: i64 = (s_16_0 + s_16_1);
        // D s_16_3: write-var u#6703 <= s_16_2
        fn_state.u_6703 = s_16_2;
        // N s_16_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var elements:i64
        let s_17_0: i64 = fn_state.elements;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: read-var offset:i
        let s_17_2: i128 = fn_state.offset;
        // D s_17_3: mul s_17_2 s_17_1
        let s_17_3: i128 = ((s_17_2) * (s_17_1));
        // D s_17_4: read-var nreg:i64
        let s_17_4: i64 = fn_state.nreg;
        // D s_17_5: cast zx s_17_4 -> i
        let s_17_5: i128 = (i128::try_from(s_17_4).unwrap());
        // D s_17_6: mul s_17_3 s_17_5
        let s_17_6: i128 = ((s_17_3) * (s_17_5));
        // D s_17_7: read-var e:i64
        let s_17_7: i64 = fn_state.e;
        // D s_17_8: cast zx s_17_7 -> i
        let s_17_8: i128 = (i128::try_from(s_17_7).unwrap());
        // D s_17_9: read-var nreg:i64
        let s_17_9: i64 = fn_state.nreg;
        // D s_17_10: cast zx s_17_9 -> i
        let s_17_10: i128 = (i128::try_from(s_17_9).unwrap());
        // D s_17_11: mul s_17_8 s_17_10
        let s_17_11: i128 = ((s_17_8) * (s_17_10));
        // D s_17_12: cast reint s_17_11 -> i64
        let s_17_12: i64 = (s_17_11 as i64);
        // D s_17_13: cast zx s_17_12 -> i
        let s_17_13: i128 = (i128::try_from(s_17_12).unwrap());
        // D s_17_14: add s_17_6 s_17_13
        let s_17_14: i128 = (s_17_6 + s_17_13);
        // D s_17_15: read-var u#6703:i64
        let s_17_15: i64 = fn_state.u_6703;
        // D s_17_16: cast zx s_17_15 -> i
        let s_17_16: i128 = (i128::try_from(s_17_15).unwrap());
        // D s_17_17: add s_17_14 s_17_16
        let s_17_17: i128 = (s_17_14 + s_17_16);
        // D s_17_18: read-var mbytes:i64
        let s_17_18: i64 = fn_state.mbytes;
        // D s_17_19: cast zx s_17_18 -> i
        let s_17_19: i128 = (i128::try_from(s_17_18).unwrap());
        // D s_17_20: mul s_17_17 s_17_19
        let s_17_20: i128 = ((s_17_17) * (s_17_19));
        // D s_17_21: read-var base:u64
        let s_17_21: u64 = fn_state.base;
        // D s_17_22: cast zx s_17_21 -> bv
        let s_17_22: Bits = Bits::new(s_17_21 as u128, 64u16);
        // D s_17_23: cast cvt s_17_20 -> bv
        let s_17_23: Bits = Bits::new(s_17_20 as u128, 128);
        // D s_17_24: add s_17_22 s_17_23
        let s_17_24: Bits = (s_17_22 + s_17_23);
        // D s_17_25: cast reint s_17_24 -> u64
        let s_17_25: u64 = (s_17_24.value() as u64);
        // D s_17_26: read-var values_name:[bv; 2]
        let s_17_26: [Bits; 2usize] = fn_state.values_name;
        // D s_17_27: cast cvt s_17_26 -> [bv; 0]
        let s_17_27: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_17_26);
        // D s_17_28: read-var u#6703:i64
        let s_17_28: i64 = fn_state.u_6703;
        // D s_17_29: cast zx s_17_28 -> i
        let s_17_29: i128 = (i128::try_from(s_17_28).unwrap());
        // D s_17_30: read-element s_17_27[s_17_29]
        let s_17_30: Bits = s_17_27[(s_17_29) as usize];
        // D s_17_31: read-var esize:i64
        let s_17_31: i64 = fn_state.esize;
        // D s_17_32: cast zx s_17_31 -> i
        let s_17_32: i128 = (i128::try_from(s_17_31).unwrap());
        // D s_17_33: cast reint s_17_32 -> i64
        let s_17_33: i64 = (s_17_32 as i64);
        // D s_17_34: read-var e:i64
        let s_17_34: i64 = fn_state.e;
        // D s_17_35: cast zx s_17_34 -> i
        let s_17_35: i128 = (i128::try_from(s_17_34).unwrap());
        // D s_17_36: cast zx s_17_33 -> i
        let s_17_36: i128 = (i128::try_from(s_17_33).unwrap());
        // D s_17_37: call Elem_read(s_17_30, s_17_35, s_17_36)
        let s_17_37: Bits = Elem_read(state, tracer, s_17_30, s_17_35, s_17_36);
        // D s_17_38: read-var mbytes:i64
        let s_17_38: i64 = fn_state.mbytes;
        // D s_17_39: cast zx s_17_38 -> i
        let s_17_39: i128 = (i128::try_from(s_17_38).unwrap());
        // D s_17_40: read-var accdesc:struct
        let s_17_40: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_17_41: call Mem_set(s_17_25, s_17_39, s_17_40, s_17_37)
        let s_17_41: () = Mem_set(state, tracer, s_17_25, s_17_39, s_17_40, s_17_37);
        // N s_17_42: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var e:i64
        let s_19_0: i64 = fn_state.e;
        // C s_19_1: const #1s : i64
        let s_19_1: i64 = 1;
        // D s_19_2: add s_19_0 s_19_1
        let s_19_2: i64 = (s_19_0 + s_19_1);
        // D s_19_3: write-var e <= s_19_2
        fn_state.e = s_19_2;
        // N s_19_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call SP_read(s_21_0)
        let s_21_1: u64 = SP_read(state, tracer, s_21_0);
        // D s_21_2: write-var base <= s_21_1
        fn_state.base = s_21_1;
        // N s_21_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call CheckSPAlignment(s_22_0)
        let s_22_1: () = CheckSPAlignment(state, tracer, s_22_0);
        // N s_22_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #31s : i
        let s_23_0: i128 = 31;
        // D s_23_1: read-var n:i64
        let s_23_1: i64 = fn_state.n;
        // D s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (i128::try_from(s_23_1).unwrap());
        // D s_23_3: cmp-eq s_23_2 s_23_0
        let s_23_3: bool = ((s_23_2) == (s_23_0));
        // N s_23_4: branch s_23_3 b29 b24
        if s_23_3 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#247206 <= s_24_0
        fn_state.gs_247206 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#247206:u8
        let s_25_0: bool = fn_state.gs_247206;
        // N s_25_1: branch s_25_0 b28 b26
        if s_25_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call CheckSPAlignment(s_28_0)
        let s_28_1: () = CheckSPAlignment(state, tracer, s_28_0);
        // N s_28_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #53u : u32
        let s_29_0: u32 = 53;
        // S s_29_1: call ConstrainUnpredictableBool(s_29_0)
        let s_29_1: bool = ConstrainUnpredictableBool(state, tracer, s_29_0);
        // D s_29_2: write-var gs#247206 <= s_29_1
        fn_state.gs_247206 = s_29_1;
        // N s_29_3: jump b25
        return block_25(state, tracer, fn_state);
    }
}
