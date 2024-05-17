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
use Zeros::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use Elem_set::*;
use ElemFFR_set::*;
use ElemFFR_read::*;
use AnyActiveElement::*;
use CreateAccDescSVEFF::*;
use ActivePredicateElement::*;
use P_read::*;
use MemNF_read::*;
use Mem_read::*;
use Z_set::*;
use common::*;
pub fn execute_LDFF1D_Z_P_AI_D<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    msize: i64,
    n: i64,
    offset: i64,
    t: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_240320: bool,
        e: i64,
        fault: bool,
        base: Bits,
        addr: u64,
        ga_310676: i64,
        unknown: bool,
        mbytes: i64,
        VLshadow_4954: i64,
        mask: Bits,
        gs_240336: bool,
        ga_310668: i64,
        faulted: bool,
        orig: Bits,
        gs_240321: bool,
        ga_310669: u64,
        data: Bits,
        baseshadow_4956: Bits,
        accdesc: ProductType9878976b5bcce9c9,
        elements: i64,
        gs_240317: i64,
        VLshadow_4955: i64,
        ga_310658: ProductTypef506aa96a892fe52,
        result: Bits,
        ga_310664: ProductTypef506aa96a892fe52,
        ga_310677: u64,
        VL: i64,
        esize: i64,
        g: i64,
        msize: i64,
        n: i64,
        offset: i64,
        t: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        msize,
        n,
        offset,
        t,
        is_unsigned,
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
        // D s_0_3: write-var VLshadow#4954 <= s_0_2
        fn_state.VLshadow_4954 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckNonStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4954:i64
        let s_1_0: i64 = fn_state.VLshadow_4954;
        // D s_1_1: write-var VLshadow#4955 <= s_1_0
        fn_state.VLshadow_4955 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4955:i64
        let s_1_3: i64 = fn_state.VLshadow_4955;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4955:i64
        let s_1_7: i64 = fn_state.VLshadow_4955;
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
        // D s_1_21: read-var VLshadow#4955:i64
        let s_1_21: i64 = fn_state.VLshadow_4955;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var t:i64
        let s_1_24: i64 = fn_state.t;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast zx s_1_23 -> i
        let s_1_26: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_27: call Z_read(s_1_25, s_1_26)
        let s_1_27: Bits = Z_read(state, tracer, s_1_25, s_1_26);
        // D s_1_28: write-var orig <= s_1_27
        fn_state.orig = s_1_27;
        // C s_1_29: const #8s : i
        let s_1_29: i128 = 8;
        // D s_1_30: read-var msize:i64
        let s_1_30: i64 = fn_state.msize;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: div s_1_31 s_1_29
        let s_1_32: i128 = ((s_1_31) / (s_1_29));
        // D s_1_33: cast reint s_1_32 -> i64
        let s_1_33: i64 = (s_1_32 as i64);
        // D s_1_34: write-var mbytes <= s_1_33
        fn_state.mbytes = s_1_33;
        // C s_1_35: const #0u : u8
        let s_1_35: bool = false;
        // D s_1_36: write-var fault <= s_1_35
        fn_state.fault = s_1_35;
        // C s_1_37: const #0u : u8
        let s_1_37: bool = false;
        // D s_1_38: write-var faulted <= s_1_37
        fn_state.faulted = s_1_37;
        // C s_1_39: const #0u : u8
        let s_1_39: bool = false;
        // D s_1_40: write-var unknown <= s_1_39
        fn_state.unknown = s_1_39;
        // C s_1_41: const #0u : u8
        let s_1_41: bool = false;
        // C s_1_42: const #1u : u8
        let s_1_42: bool = true;
        // S s_1_43: call CreateAccDescSVEFF(s_1_41, s_1_42)
        let s_1_43: ProductType9878976b5bcce9c9 = CreateAccDescSVEFF(
            state,
            tracer,
            s_1_41,
            s_1_42,
        );
        // D s_1_44: write-var accdesc <= s_1_43
        fn_state.accdesc = s_1_43;
        // D s_1_45: read-var esize:i64
        let s_1_45: i64 = fn_state.esize;
        // D s_1_46: cast zx s_1_45 -> i
        let s_1_46: i128 = (i128::try_from(s_1_45).unwrap());
        // D s_1_47: read-var mask:bv
        let s_1_47: Bits = fn_state.mask;
        // D s_1_48: call AnyActiveElement(s_1_47, s_1_46)
        let s_1_48: bool = AnyActiveElement(state, tracer, s_1_47, s_1_46);
        // N s_1_49: branch s_1_48 b39 b2
        if s_1_48 {
            return block_39(state, tracer, fn_state);
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
        // D s_3_0: read-var base:bv
        let s_3_0: Bits = fn_state.base;
        // D s_3_1: write-var baseshadow#4956 <= s_3_0
        fn_state.baseshadow_4956 = s_3_0;
        // D s_3_2: read-var accdesc.10:struct
        let s_3_2: bool = fn_state.accdesc._10;
        // N s_3_3: assert s_3_2
        let s_3_3: () = assert!(s_3_2);
        // C s_3_4: const #0s : i64
        let s_3_4: i64 = 0;
        // C s_3_5: const #1s : i
        let s_3_5: i128 = 1;
        // D s_3_6: read-var elements:i64
        let s_3_6: i64 = fn_state.elements;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: sub s_3_7 s_3_5
        let s_3_8: i128 = ((s_3_7) - (s_3_5));
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: write-var gs#240317 <= s_3_9
        fn_state.gs_240317 = s_3_9;
        // D s_3_11: write-var e <= s_3_4
        fn_state.e = s_3_4;
        // N s_3_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#240317:i64
        let s_4_1: i64 = fn_state.gs_240317;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b38 b5
        if s_4_2 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var esize:i64
        let s_5_2: i64 = fn_state.esize;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var mask:bv
        let s_5_4: Bits = fn_state.mask;
        // D s_5_5: call ActivePredicateElement(s_5_4, s_5_1, s_5_3)
        let s_5_5: bool = ActivePredicateElement(state, tracer, s_5_4, s_5_1, s_5_3);
        // N s_5_6: branch s_5_5 b33 b6
        if s_5_5 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var msize:i64
        let s_6_0: i64 = fn_state.msize;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call Zeros(s_6_1)
        let s_6_2: Bits = Zeros(state, tracer, s_6_1);
        // C s_6_3: const #0u : u8
        let s_6_3: bool = false;
        // D s_6_4: create-product struct = ["s_6_2", "s_6_3"]
        let s_6_4: ProductTypef506aa96a892fe52 = ProductTypef506aa96a892fe52 {
            _0: s_6_2,
            _1: s_6_3,
        };
        // D s_6_5: write-var ga#310664 <= s_6_4
        fn_state.ga_310664 = s_6_4;
        // D s_6_6: read-var ga#310664.0:struct
        let s_6_6: Bits = fn_state.ga_310664._0;
        // D s_6_7: read-var ga#310664.1:struct
        let s_6_7: bool = fn_state.ga_310664._1;
        // D s_6_8: write-var data <= s_6_6
        fn_state.data = s_6_6;
        // D s_6_9: write-var fault <= s_6_7
        fn_state.fault = s_6_7;
        // N s_6_10: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var faulted:u8
        let s_7_0: bool = fn_state.faulted;
        // N s_7_1: branch s_7_0 b32 b8
        if s_7_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var fault:u8
        let s_8_0: bool = fn_state.fault;
        // D s_8_1: write-var gs#240320 <= s_8_0
        fn_state.gs_240320 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#240320:u8
        let s_9_0: bool = fn_state.gs_240320;
        // D s_9_1: write-var faulted <= s_9_0
        fn_state.faulted = s_9_0;
        // D s_9_2: read-var faulted:u8
        let s_9_2: bool = fn_state.faulted;
        // N s_9_3: branch s_9_2 b31 b10
        if s_9_2 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var unknown:u8
        let s_11_0: bool = fn_state.unknown;
        // N s_11_1: branch s_11_0 b30 b12
        if s_11_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var e:i64
        let s_12_0: i64 = fn_state.e;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var esize:i64
        let s_12_2: i64 = fn_state.esize;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: call ElemFFR_read(s_12_1, s_12_3)
        let s_12_4: bool = ElemFFR_read(state, tracer, s_12_1, s_12_3);
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // C s_12_6: const #0u : u8
        let s_12_6: bool = false;
        // C s_12_7: cast zx s_12_6 -> bv
        let s_12_7: Bits = Bits::new(s_12_6 as u128, 1u16);
        // D s_12_8: cmp-eq s_12_5 s_12_7
        let s_12_8: bool = ((s_12_5) == (s_12_7));
        // D s_12_9: write-var gs#240321 <= s_12_8
        fn_state.gs_240321 = s_12_8;
        // N s_12_10: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#240321:u8
        let s_13_0: bool = fn_state.gs_240321;
        // D s_13_1: write-var unknown <= s_13_0
        fn_state.unknown = s_13_0;
        // D s_13_2: read-var unknown:u8
        let s_13_2: bool = fn_state.unknown;
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
        // D s_14_0: read-var esize:i64
        let s_14_0: i64 = fn_state.esize;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: cast reint s_14_1 -> i64
        let s_14_2: i64 = (s_14_1 as i64);
        // D s_14_3: write-var ga#310676 <= s_14_2
        fn_state.ga_310676 = s_14_2;
        // D s_14_4: read-var is_unsigned:u8
        let s_14_4: bool = fn_state.is_unsigned;
        // N s_14_5: branch s_14_4 b18 b15
        if s_14_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esize:i64
        let s_15_0: i64 = fn_state.esize;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: read-var data:bv
        let s_15_2: Bits = fn_state.data;
        // D s_15_3: bits-cast sx s_15_2 -> bv length s_15_1
        let s_15_3: Bits = s_15_2.sign_extend(s_15_1);
        // D s_15_4: cast reint s_15_3 -> u64
        let s_15_4: u64 = (s_15_3.value() as u64);
        // D s_15_5: write-var ga#310677 <= s_15_4
        fn_state.ga_310677 = s_15_4;
        // N s_15_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var e:i64
        let s_16_0: i64 = fn_state.e;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: read-var ga#310676:i64
        let s_16_2: i64 = fn_state.ga_310676;
        // D s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_4: read-var ga#310677:u64
        let s_16_4: u64 = fn_state.ga_310677;
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 64u16);
        // D s_16_6: read-var result:bv
        let s_16_6: Bits = fn_state.result;
        // D s_16_7: call Elem_set(s_16_6, s_16_1, s_16_3, s_16_5)
        let s_16_7: Bits = Elem_set(state, tracer, s_16_6, s_16_1, s_16_3, s_16_5);
        // D s_16_8: write-var result <= s_16_7
        fn_state.result = s_16_7;
        // N s_16_9: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var e:i64
        let s_17_0: i64 = fn_state.e;
        // C s_17_1: const #1s : i64
        let s_17_1: i64 = 1;
        // D s_17_2: add s_17_0 s_17_1
        let s_17_2: i64 = (s_17_0 + s_17_1);
        // D s_17_3: write-var e <= s_17_2
        fn_state.e = s_17_2;
        // N s_17_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var esize:i64
        let s_18_0: i64 = fn_state.esize;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: read-var data:bv
        let s_18_2: Bits = fn_state.data;
        // D s_18_3: bits-cast zx s_18_2 -> bv length s_18_1
        let s_18_3: Bits = s_18_2.zero_extend(s_18_1);
        // D s_18_4: cast reint s_18_3 -> u64
        let s_18_4: u64 = (s_18_3.value() as u64);
        // D s_18_5: write-var ga#310677 <= s_18_4
        fn_state.ga_310677 = s_18_4;
        // N s_18_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var fault:u8
        let s_19_0: bool = fn_state.fault;
        // D s_19_1: not s_19_0
        let s_19_1: bool = !s_19_0;
        // N s_19_2: branch s_19_1 b29 b20
        if s_19_1 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#240336 <= s_20_0
        fn_state.gs_240336 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#240336:u8
        let s_21_0: bool = fn_state.gs_240336;
        // N s_21_1: branch s_21_0 b25 b22
        if s_21_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #52u : u32
        let s_22_0: u32 = 52;
        // S s_22_1: call ConstrainUnpredictableBool(s_22_0)
        let s_22_1: bool = ConstrainUnpredictableBool(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b24 b23
        if s_22_1 {
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
        // D s_23_0: read-var esize:i64
        let s_23_0: i64 = fn_state.esize;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: cast reint s_23_1 -> i64
        let s_23_2: i64 = (s_23_1 as i64);
        // D s_23_3: read-var esize:i64
        let s_23_3: i64 = fn_state.esize;
        // D s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_5: cast reint s_23_4 -> i64
        let s_23_5: i64 = (s_23_4 as i64);
        // D s_23_6: read-var e:i64
        let s_23_6: i64 = fn_state.e;
        // D s_23_7: cast zx s_23_6 -> i
        let s_23_7: i128 = (i128::try_from(s_23_6).unwrap());
        // D s_23_8: cast zx s_23_5 -> i
        let s_23_8: i128 = (i128::try_from(s_23_5).unwrap());
        // D s_23_9: read-var orig:bv
        let s_23_9: Bits = fn_state.orig;
        // D s_23_10: call Elem_read(s_23_9, s_23_7, s_23_8)
        let s_23_10: Bits = Elem_read(state, tracer, s_23_9, s_23_7, s_23_8);
        // D s_23_11: cast reint s_23_10 -> u64
        let s_23_11: u64 = (s_23_10.value() as u64);
        // D s_23_12: read-var e:i64
        let s_23_12: i64 = fn_state.e;
        // D s_23_13: cast zx s_23_12 -> i
        let s_23_13: i128 = (i128::try_from(s_23_12).unwrap());
        // D s_23_14: cast zx s_23_2 -> i
        let s_23_14: i128 = (i128::try_from(s_23_2).unwrap());
        // D s_23_15: cast zx s_23_11 -> bv
        let s_23_15: Bits = Bits::new(s_23_11 as u128, 64u16);
        // D s_23_16: read-var result:bv
        let s_23_16: Bits = fn_state.result;
        // D s_23_17: call Elem_set(s_23_16, s_23_13, s_23_14, s_23_15)
        let s_23_17: Bits = Elem_set(state, tracer, s_23_16, s_23_13, s_23_14, s_23_15);
        // D s_23_18: write-var result <= s_23_17
        fn_state.result = s_23_17;
        // N s_23_19: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var esize:i64
        let s_24_0: i64 = fn_state.esize;
        // D s_24_1: cast zx s_24_0 -> i
        let s_24_1: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_2: cast reint s_24_1 -> i64
        let s_24_2: i64 = (s_24_1 as i64);
        // D s_24_3: read-var esize:i64
        let s_24_3: i64 = fn_state.esize;
        // D s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // D s_24_5: call Zeros(s_24_4)
        let s_24_5: Bits = Zeros(state, tracer, s_24_4);
        // D s_24_6: cast reint s_24_5 -> u64
        let s_24_6: u64 = (s_24_5.value() as u64);
        // D s_24_7: read-var e:i64
        let s_24_7: i64 = fn_state.e;
        // D s_24_8: cast zx s_24_7 -> i
        let s_24_8: i128 = (i128::try_from(s_24_7).unwrap());
        // D s_24_9: cast zx s_24_2 -> i
        let s_24_9: i128 = (i128::try_from(s_24_2).unwrap());
        // D s_24_10: cast zx s_24_6 -> bv
        let s_24_10: Bits = Bits::new(s_24_6 as u128, 64u16);
        // D s_24_11: read-var result:bv
        let s_24_11: Bits = fn_state.result;
        // D s_24_12: call Elem_set(s_24_11, s_24_8, s_24_9, s_24_10)
        let s_24_12: Bits = Elem_set(state, tracer, s_24_11, s_24_8, s_24_9, s_24_10);
        // D s_24_13: write-var result <= s_24_12
        fn_state.result = s_24_12;
        // N s_24_14: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var esize:i64
        let s_25_0: i64 = fn_state.esize;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: cast reint s_25_1 -> i64
        let s_25_2: i64 = (s_25_1 as i64);
        // D s_25_3: write-var ga#310668 <= s_25_2
        fn_state.ga_310668 = s_25_2;
        // D s_25_4: read-var is_unsigned:u8
        let s_25_4: bool = fn_state.is_unsigned;
        // N s_25_5: branch s_25_4 b28 b26
        if s_25_4 {
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
        // D s_26_0: read-var esize:i64
        let s_26_0: i64 = fn_state.esize;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: read-var data:bv
        let s_26_2: Bits = fn_state.data;
        // D s_26_3: bits-cast sx s_26_2 -> bv length s_26_1
        let s_26_3: Bits = s_26_2.sign_extend(s_26_1);
        // D s_26_4: cast reint s_26_3 -> u64
        let s_26_4: u64 = (s_26_3.value() as u64);
        // D s_26_5: write-var ga#310669 <= s_26_4
        fn_state.ga_310669 = s_26_4;
        // N s_26_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var e:i64
        let s_27_0: i64 = fn_state.e;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: read-var ga#310668:i64
        let s_27_2: i64 = fn_state.ga_310668;
        // D s_27_3: cast zx s_27_2 -> i
        let s_27_3: i128 = (i128::try_from(s_27_2).unwrap());
        // D s_27_4: read-var ga#310669:u64
        let s_27_4: u64 = fn_state.ga_310669;
        // D s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 64u16);
        // D s_27_6: read-var result:bv
        let s_27_6: Bits = fn_state.result;
        // D s_27_7: call Elem_set(s_27_6, s_27_1, s_27_3, s_27_5)
        let s_27_7: Bits = Elem_set(state, tracer, s_27_6, s_27_1, s_27_3, s_27_5);
        // D s_27_8: write-var result <= s_27_7
        fn_state.result = s_27_7;
        // N s_27_9: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var esize:i64
        let s_28_0: i64 = fn_state.esize;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: read-var data:bv
        let s_28_2: Bits = fn_state.data;
        // D s_28_3: bits-cast zx s_28_2 -> bv length s_28_1
        let s_28_3: Bits = s_28_2.zero_extend(s_28_1);
        // D s_28_4: cast reint s_28_3 -> u64
        let s_28_4: u64 = (s_28_3.value() as u64);
        // D s_28_5: write-var ga#310669 <= s_28_4
        fn_state.ga_310669 = s_28_4;
        // N s_28_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #51u : u32
        let s_29_0: u32 = 51;
        // S s_29_1: call ConstrainUnpredictableBool(s_29_0)
        let s_29_1: bool = ConstrainUnpredictableBool(state, tracer, s_29_0);
        // D s_29_2: write-var gs#240336 <= s_29_1
        fn_state.gs_240336 = s_29_1;
        // N s_29_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#240321 <= s_30_0
        fn_state.gs_240321 = s_30_0;
        // N s_30_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var e:i64
        let s_31_0: i64 = fn_state.e;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: read-var esize:i64
        let s_31_2: i64 = fn_state.esize;
        // D s_31_3: cast zx s_31_2 -> i
        let s_31_3: i128 = (i128::try_from(s_31_2).unwrap());
        // C s_31_4: const #0u : u8
        let s_31_4: bool = false;
        // D s_31_5: call ElemFFR_set(s_31_1, s_31_3, s_31_4)
        let s_31_5: () = ElemFFR_set(state, tracer, s_31_1, s_31_3, s_31_4);
        // N s_31_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#240320 <= s_32_0
        fn_state.gs_240320 = s_32_0;
        // N s_32_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var esize:i64
        let s_33_0: i64 = fn_state.esize;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: cast reint s_33_1 -> i64
        let s_33_2: i64 = (s_33_1 as i64);
        // D s_33_3: read-var e:i64
        let s_33_3: i64 = fn_state.e;
        // D s_33_4: cast zx s_33_3 -> i
        let s_33_4: i128 = (i128::try_from(s_33_3).unwrap());
        // D s_33_5: cast zx s_33_2 -> i
        let s_33_5: i128 = (i128::try_from(s_33_2).unwrap());
        // D s_33_6: read-var baseshadow#4956:bv
        let s_33_6: Bits = fn_state.baseshadow_4956;
        // D s_33_7: call Elem_read(s_33_6, s_33_4, s_33_5)
        let s_33_7: Bits = Elem_read(state, tracer, s_33_6, s_33_4, s_33_5);
        // D s_33_8: cast reint s_33_7 -> u64
        let s_33_8: u64 = (s_33_7.value() as u64);
        // C s_33_9: const #64s : i
        let s_33_9: i128 = 64;
        // D s_33_10: cast zx s_33_8 -> bv
        let s_33_10: Bits = Bits::new(s_33_8 as u128, 64u16);
        // D s_33_11: bits-cast zx s_33_10 -> bv length s_33_9
        let s_33_11: Bits = s_33_10.zero_extend(s_33_9);
        // D s_33_12: cast reint s_33_11 -> u64
        let s_33_12: u64 = (s_33_11.value() as u64);
        // D s_33_13: read-var offset:i64
        let s_33_13: i64 = fn_state.offset;
        // D s_33_14: cast zx s_33_13 -> i
        let s_33_14: i128 = (i128::try_from(s_33_13).unwrap());
        // D s_33_15: read-var mbytes:i64
        let s_33_15: i64 = fn_state.mbytes;
        // D s_33_16: cast zx s_33_15 -> i
        let s_33_16: i128 = (i128::try_from(s_33_15).unwrap());
        // D s_33_17: mul s_33_14 s_33_16
        let s_33_17: i128 = ((s_33_14) * (s_33_16));
        // D s_33_18: cast reint s_33_17 -> i64
        let s_33_18: i64 = (s_33_17 as i64);
        // D s_33_19: cast zx s_33_12 -> bv
        let s_33_19: Bits = Bits::new(s_33_12 as u128, 64u16);
        // D s_33_20: cast zx s_33_18 -> i
        let s_33_20: i128 = (i128::try_from(s_33_18).unwrap());
        // D s_33_21: cast cvt s_33_20 -> bv
        let s_33_21: Bits = Bits::new(s_33_20 as u128, 128);
        // D s_33_22: add s_33_19 s_33_21
        let s_33_22: Bits = (s_33_19 + s_33_21);
        // D s_33_23: cast reint s_33_22 -> u64
        let s_33_23: u64 = (s_33_22.value() as u64);
        // D s_33_24: write-var addr <= s_33_23
        fn_state.addr = s_33_23;
        // D s_33_25: read-var accdesc.10:struct
        let s_33_25: bool = fn_state.accdesc._10;
        // N s_33_26: branch s_33_25 b36 b34
        if s_33_25 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var addr:u64
        let s_34_0: u64 = fn_state.addr;
        // D s_34_1: read-var mbytes:i64
        let s_34_1: i64 = fn_state.mbytes;
        // D s_34_2: read-var accdesc:struct
        let s_34_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_34_3: call MemNF_read(s_34_0, s_34_1, s_34_2)
        let s_34_3: ProductTypef506aa96a892fe52 = MemNF_read(
            state,
            tracer,
            s_34_0,
            s_34_1,
            s_34_2,
        );
        // D s_34_4: write-var ga#310658 <= s_34_3
        fn_state.ga_310658 = s_34_3;
        // N s_34_5: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var ga#310658.0:struct
        let s_35_0: Bits = fn_state.ga_310658._0;
        // D s_35_1: read-var ga#310658.1:struct
        let s_35_1: bool = fn_state.ga_310658._1;
        // D s_35_2: write-var data <= s_35_0
        fn_state.data = s_35_0;
        // D s_35_3: write-var fault <= s_35_1
        fn_state.fault = s_35_1;
        // N s_35_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var mbytes:i64
        let s_36_0: i64 = fn_state.mbytes;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: read-var addr:u64
        let s_36_2: u64 = fn_state.addr;
        // D s_36_3: read-var accdesc:struct
        let s_36_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_36_4: call Mem_read(s_36_2, s_36_1, s_36_3)
        let s_36_4: Bits = Mem_read(state, tracer, s_36_2, s_36_1, s_36_3);
        // D s_36_5: write-var data <= s_36_4
        fn_state.data = s_36_4;
        // N s_36_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var accdesc.10 <= s_37_0
        fn_state.accdesc._10 = s_37_0;
        // N s_37_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var VLshadow#4955:i64
        let s_38_0: i64 = fn_state.VLshadow_4955;
        // D s_38_1: cast zx s_38_0 -> i
        let s_38_1: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_2: cast reint s_38_1 -> i64
        let s_38_2: i64 = (s_38_1 as i64);
        // D s_38_3: read-var t:i64
        let s_38_3: i64 = fn_state.t;
        // D s_38_4: cast zx s_38_3 -> i
        let s_38_4: i128 = (i128::try_from(s_38_3).unwrap());
        // D s_38_5: cast zx s_38_2 -> i
        let s_38_5: i128 = (i128::try_from(s_38_2).unwrap());
        // D s_38_6: read-var result:bv
        let s_38_6: Bits = fn_state.result;
        // D s_38_7: call Z_set(s_38_4, s_38_5, s_38_6)
        let s_38_7: () = Z_set(state, tracer, s_38_4, s_38_5, s_38_6);
        // N s_38_8: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var VLshadow#4955:i64
        let s_39_0: i64 = fn_state.VLshadow_4955;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: cast reint s_39_1 -> i64
        let s_39_2: i64 = (s_39_1 as i64);
        // D s_39_3: read-var n:i64
        let s_39_3: i64 = fn_state.n;
        // D s_39_4: cast zx s_39_3 -> i
        let s_39_4: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_5: cast zx s_39_2 -> i
        let s_39_5: i128 = (i128::try_from(s_39_2).unwrap());
        // D s_39_6: call Z_read(s_39_4, s_39_5)
        let s_39_6: Bits = Z_read(state, tracer, s_39_4, s_39_5);
        // D s_39_7: write-var base <= s_39_6
        fn_state.base = s_39_6;
        // N s_39_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
