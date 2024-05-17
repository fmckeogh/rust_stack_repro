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
use Elem_set::*;
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use AddWithCarry::*;
use Z_set::*;
use common::*;
pub fn execute_SBCLT_Z_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        p: i64,
        VLshadow_4309: i64,
        VLshadow_4308: i64,
        carries: Bits,
        gs_218701: i64,
        esizeshadow_4307: i64,
        result: Bits,
        ga_298502: ProductTyped54bc449dd09e5bd,
        VL: i64,
        da: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
        m,
        n,
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
        // D s_0_3: write-var esizeshadow#4307 <= s_0_2
        fn_state.esizeshadow_4307 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#4308 <= s_0_6
        fn_state.VLshadow_4308 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSVEEnabled(s_0_8)
        let s_0_9: () = CheckSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4308:i64
        let s_1_0: i64 = fn_state.VLshadow_4308;
        // D s_1_1: write-var VLshadow#4309 <= s_1_0
        fn_state.VLshadow_4309 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esizeshadow#4307:i64
        let s_1_3: i64 = fn_state.esizeshadow_4307;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) * (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4309:i64
        let s_1_7: i64 = fn_state.VLshadow_4309;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var VLshadow#4309:i64
        let s_1_12: i64 = fn_state.VLshadow_4309;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var n:i64
        let s_1_15: i64 = fn_state.n;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call Z_read(s_1_16, s_1_17)
        let s_1_18: Bits = Z_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var operand <= s_1_18
        fn_state.operand = s_1_18;
        // D s_1_20: read-var VLshadow#4309:i64
        let s_1_20: i64 = fn_state.VLshadow_4309;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var m:i64
        let s_1_23: i64 = fn_state.m;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_22 -> i
        let s_1_25: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_26: call Z_read(s_1_24, s_1_25)
        let s_1_26: Bits = Z_read(state, tracer, s_1_24, s_1_25);
        // D s_1_27: write-var carries <= s_1_26
        fn_state.carries = s_1_26;
        // D s_1_28: read-var VLshadow#4309:i64
        let s_1_28: i64 = fn_state.VLshadow_4309;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: read-var da:i64
        let s_1_31: i64 = fn_state.da;
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_33: cast zx s_1_30 -> i
        let s_1_33: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_34: call Z_read(s_1_32, s_1_33)
        let s_1_34: Bits = Z_read(state, tracer, s_1_32, s_1_33);
        // D s_1_35: write-var result <= s_1_34
        fn_state.result = s_1_34;
        // C s_1_36: const #0s : i64
        let s_1_36: i64 = 0;
        // C s_1_37: const #1s : i
        let s_1_37: i128 = 1;
        // D s_1_38: cast zx s_1_11 -> i
        let s_1_38: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_39: sub s_1_38 s_1_37
        let s_1_39: i128 = ((s_1_38) - (s_1_37));
        // D s_1_40: cast reint s_1_39 -> i64
        let s_1_40: i64 = (s_1_39 as i64);
        // D s_1_41: write-var gs#218701 <= s_1_40
        fn_state.gs_218701 = s_1_40;
        // D s_1_42: write-var p <= s_1_36
        fn_state.p = s_1_36;
        // N s_1_43: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var p:i64
        let s_2_0: i64 = fn_state.p;
        // D s_2_1: read-var gs#218701:i64
        let s_2_1: i64 = fn_state.gs_218701;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
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
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var p:i64
        let s_3_1: i64 = fn_state.p;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // C s_3_5: const #0s : i
        let s_3_5: i128 = 0;
        // D s_3_6: cast zx s_3_4 -> i
        let s_3_6: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_7: add s_3_6 s_3_5
        let s_3_7: i128 = (s_3_6 + s_3_5);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: read-var esizeshadow#4307:i64
        let s_3_9: i64 = fn_state.esizeshadow_4307;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: cast zx s_3_8 -> i
        let s_3_12: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_13: cast zx s_3_11 -> i
        let s_3_13: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_14: read-var result:bv
        let s_3_14: Bits = fn_state.result;
        // D s_3_15: call Elem_read(s_3_14, s_3_12, s_3_13)
        let s_3_15: Bits = Elem_read(state, tracer, s_3_14, s_3_12, s_3_13);
        // C s_3_16: const #2s : i
        let s_3_16: i128 = 2;
        // D s_3_17: read-var p:i64
        let s_3_17: i64 = fn_state.p;
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: mul s_3_16 s_3_18
        let s_3_19: i128 = ((s_3_16) * (s_3_18));
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // C s_3_21: const #1s : i
        let s_3_21: i128 = 1;
        // D s_3_22: cast zx s_3_20 -> i
        let s_3_22: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_23: add s_3_22 s_3_21
        let s_3_23: i128 = (s_3_22 + s_3_21);
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: read-var esizeshadow#4307:i64
        let s_3_25: i64 = fn_state.esizeshadow_4307;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // D s_3_28: cast zx s_3_24 -> i
        let s_3_28: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_29: cast zx s_3_27 -> i
        let s_3_29: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_30: read-var operand:bv
        let s_3_30: Bits = fn_state.operand;
        // D s_3_31: call Elem_read(s_3_30, s_3_28, s_3_29)
        let s_3_31: Bits = Elem_read(state, tracer, s_3_30, s_3_28, s_3_29);
        // C s_3_32: const #2s : i
        let s_3_32: i128 = 2;
        // D s_3_33: read-var p:i64
        let s_3_33: i64 = fn_state.p;
        // D s_3_34: cast zx s_3_33 -> i
        let s_3_34: i128 = (i128::try_from(s_3_33).unwrap());
        // D s_3_35: mul s_3_32 s_3_34
        let s_3_35: i128 = ((s_3_32) * (s_3_34));
        // D s_3_36: cast reint s_3_35 -> i64
        let s_3_36: i64 = (s_3_35 as i64);
        // C s_3_37: const #1s : i
        let s_3_37: i128 = 1;
        // D s_3_38: cast zx s_3_36 -> i
        let s_3_38: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_39: add s_3_38 s_3_37
        let s_3_39: i128 = (s_3_38 + s_3_37);
        // D s_3_40: cast reint s_3_39 -> i64
        let s_3_40: i64 = (s_3_39 as i64);
        // D s_3_41: read-var esizeshadow#4307:i64
        let s_3_41: i64 = fn_state.esizeshadow_4307;
        // D s_3_42: cast zx s_3_41 -> i
        let s_3_42: i128 = (i128::try_from(s_3_41).unwrap());
        // D s_3_43: cast reint s_3_42 -> i64
        let s_3_43: i64 = (s_3_42 as i64);
        // D s_3_44: cast zx s_3_40 -> i
        let s_3_44: i128 = (i128::try_from(s_3_40).unwrap());
        // D s_3_45: cast zx s_3_43 -> i
        let s_3_45: i128 = (i128::try_from(s_3_43).unwrap());
        // D s_3_46: read-var carries:bv
        let s_3_46: Bits = fn_state.carries;
        // D s_3_47: call Elem_read(s_3_46, s_3_44, s_3_45)
        let s_3_47: Bits = Elem_read(state, tracer, s_3_46, s_3_44, s_3_45);
        // C s_3_48: const #0s : i
        let s_3_48: i128 = 0;
        // C s_3_49: const #1u : u64
        let s_3_49: u64 = 1;
        // D s_3_50: bit-extract s_3_47 s_3_48 s_3_49
        let s_3_50: Bits = (Bits::new(
            ((s_3_47) >> (s_3_48)).value(),
            u16::try_from(s_3_49).unwrap(),
        ));
        // D s_3_51: cast reint s_3_50 -> u8
        let s_3_51: bool = ((s_3_50.value()) != 0);
        // C s_3_52: const #0s : i
        let s_3_52: i128 = 0;
        // C s_3_53: const #0u : u64
        let s_3_53: u64 = 0;
        // D s_3_54: cast zx s_3_51 -> u64
        let s_3_54: u64 = (s_3_51 as u64);
        // C s_3_55: const #1u : u64
        let s_3_55: u64 = 1;
        // D s_3_56: and s_3_54 s_3_55
        let s_3_56: u64 = ((s_3_54) & (s_3_55));
        // D s_3_57: cmp-eq s_3_56 s_3_55
        let s_3_57: bool = ((s_3_56) == (s_3_55));
        // D s_3_58: lsl s_3_54 s_3_52
        let s_3_58: u64 = s_3_54 << s_3_52;
        // D s_3_59: or s_3_53 s_3_58
        let s_3_59: u64 = ((s_3_53) | (s_3_58));
        // D s_3_60: cmpl s_3_58
        let s_3_60: u64 = !s_3_58;
        // D s_3_61: and s_3_53 s_3_60
        let s_3_61: u64 = ((s_3_53) & (s_3_60));
        // D s_3_62: select s_3_57 s_3_59 s_3_61
        let s_3_62: u64 = if s_3_57 { s_3_59 } else { s_3_61 };
        // D s_3_63: cast trunc s_3_62 -> u8
        let s_3_63: bool = ((s_3_62) != 0);
        // D s_3_64: not s_3_31
        let s_3_64: Bits = !s_3_31;
        // D s_3_65: call AddWithCarry(s_3_15, s_3_64, s_3_63)
        let s_3_65: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_3_15,
            s_3_64,
            s_3_63,
        );
        // D s_3_66: write-var ga#298502 <= s_3_65
        fn_state.ga_298502 = s_3_65;
        // D s_3_67: read-var ga#298502.0:struct
        let s_3_67: Bits = fn_state.ga_298502._0;
        // D s_3_68: read-var ga#298502.1:struct
        let s_3_68: u8 = fn_state.ga_298502._1;
        // C s_3_69: const #1s : i
        let s_3_69: i128 = 1;
        // D s_3_70: cast zx s_3_68 -> bv
        let s_3_70: Bits = Bits::new(s_3_68 as u128, 4u16);
        // C s_3_71: const #1u : u64
        let s_3_71: u64 = 1;
        // D s_3_72: bit-extract s_3_70 s_3_69 s_3_71
        let s_3_72: Bits = (Bits::new(
            ((s_3_70) >> (s_3_69)).value(),
            u16::try_from(s_3_71).unwrap(),
        ));
        // D s_3_73: cast reint s_3_72 -> u8
        let s_3_73: bool = ((s_3_72.value()) != 0);
        // C s_3_74: const #0s : i
        let s_3_74: i128 = 0;
        // C s_3_75: const #0u : u64
        let s_3_75: u64 = 0;
        // D s_3_76: cast zx s_3_73 -> u64
        let s_3_76: u64 = (s_3_73 as u64);
        // C s_3_77: const #1u : u64
        let s_3_77: u64 = 1;
        // D s_3_78: and s_3_76 s_3_77
        let s_3_78: u64 = ((s_3_76) & (s_3_77));
        // D s_3_79: cmp-eq s_3_78 s_3_77
        let s_3_79: bool = ((s_3_78) == (s_3_77));
        // D s_3_80: lsl s_3_76 s_3_74
        let s_3_80: u64 = s_3_76 << s_3_74;
        // D s_3_81: or s_3_75 s_3_80
        let s_3_81: u64 = ((s_3_75) | (s_3_80));
        // D s_3_82: cmpl s_3_80
        let s_3_82: u64 = !s_3_80;
        // D s_3_83: and s_3_75 s_3_82
        let s_3_83: u64 = ((s_3_75) & (s_3_82));
        // D s_3_84: select s_3_79 s_3_81 s_3_83
        let s_3_84: u64 = if s_3_79 { s_3_81 } else { s_3_83 };
        // D s_3_85: cast trunc s_3_84 -> u8
        let s_3_85: bool = ((s_3_84) != 0);
        // C s_3_86: const #2s : i
        let s_3_86: i128 = 2;
        // D s_3_87: read-var p:i64
        let s_3_87: i64 = fn_state.p;
        // D s_3_88: cast zx s_3_87 -> i
        let s_3_88: i128 = (i128::try_from(s_3_87).unwrap());
        // D s_3_89: mul s_3_86 s_3_88
        let s_3_89: i128 = ((s_3_86) * (s_3_88));
        // D s_3_90: cast reint s_3_89 -> i64
        let s_3_90: i64 = (s_3_89 as i64);
        // C s_3_91: const #0s : i
        let s_3_91: i128 = 0;
        // D s_3_92: cast zx s_3_90 -> i
        let s_3_92: i128 = (i128::try_from(s_3_90).unwrap());
        // D s_3_93: add s_3_92 s_3_91
        let s_3_93: i128 = (s_3_92 + s_3_91);
        // D s_3_94: cast reint s_3_93 -> i64
        let s_3_94: i64 = (s_3_93 as i64);
        // D s_3_95: read-var esizeshadow#4307:i64
        let s_3_95: i64 = fn_state.esizeshadow_4307;
        // D s_3_96: cast zx s_3_95 -> i
        let s_3_96: i128 = (i128::try_from(s_3_95).unwrap());
        // D s_3_97: cast reint s_3_96 -> i64
        let s_3_97: i64 = (s_3_96 as i64);
        // D s_3_98: cast zx s_3_94 -> i
        let s_3_98: i128 = (i128::try_from(s_3_94).unwrap());
        // D s_3_99: cast zx s_3_97 -> i
        let s_3_99: i128 = (i128::try_from(s_3_97).unwrap());
        // D s_3_100: read-var result:bv
        let s_3_100: Bits = fn_state.result;
        // D s_3_101: call Elem_set(s_3_100, s_3_98, s_3_99, s_3_67)
        let s_3_101: Bits = Elem_set(state, tracer, s_3_100, s_3_98, s_3_99, s_3_67);
        // D s_3_102: write-var result <= s_3_101
        fn_state.result = s_3_101;
        // C s_3_103: const #2s : i
        let s_3_103: i128 = 2;
        // D s_3_104: read-var p:i64
        let s_3_104: i64 = fn_state.p;
        // D s_3_105: cast zx s_3_104 -> i
        let s_3_105: i128 = (i128::try_from(s_3_104).unwrap());
        // D s_3_106: mul s_3_103 s_3_105
        let s_3_106: i128 = ((s_3_103) * (s_3_105));
        // D s_3_107: cast reint s_3_106 -> i64
        let s_3_107: i64 = (s_3_106 as i64);
        // C s_3_108: const #1s : i
        let s_3_108: i128 = 1;
        // D s_3_109: cast zx s_3_107 -> i
        let s_3_109: i128 = (i128::try_from(s_3_107).unwrap());
        // D s_3_110: add s_3_109 s_3_108
        let s_3_110: i128 = (s_3_109 + s_3_108);
        // D s_3_111: cast reint s_3_110 -> i64
        let s_3_111: i64 = (s_3_110 as i64);
        // D s_3_112: read-var esizeshadow#4307:i64
        let s_3_112: i64 = fn_state.esizeshadow_4307;
        // D s_3_113: cast zx s_3_112 -> i
        let s_3_113: i128 = (i128::try_from(s_3_112).unwrap());
        // D s_3_114: cast reint s_3_113 -> i64
        let s_3_114: i64 = (s_3_113 as i64);
        // D s_3_115: cast zx s_3_85 -> bv
        let s_3_115: Bits = Bits::new(s_3_85 as u128, 1u16);
        // D s_3_116: read-var esizeshadow#4307:i64
        let s_3_116: i64 = fn_state.esizeshadow_4307;
        // D s_3_117: cast zx s_3_116 -> i
        let s_3_117: i128 = (i128::try_from(s_3_116).unwrap());
        // D s_3_118: bits-cast zx s_3_115 -> bv length s_3_117
        let s_3_118: Bits = s_3_115.zero_extend(s_3_117);
        // D s_3_119: cast zx s_3_111 -> i
        let s_3_119: i128 = (i128::try_from(s_3_111).unwrap());
        // D s_3_120: cast zx s_3_114 -> i
        let s_3_120: i128 = (i128::try_from(s_3_114).unwrap());
        // D s_3_121: read-var result:bv
        let s_3_121: Bits = fn_state.result;
        // D s_3_122: call Elem_set(s_3_121, s_3_119, s_3_120, s_3_118)
        let s_3_122: Bits = Elem_set(state, tracer, s_3_121, s_3_119, s_3_120, s_3_118);
        // D s_3_123: write-var result <= s_3_122
        fn_state.result = s_3_122;
        // D s_3_124: read-var p:i64
        let s_3_124: i64 = fn_state.p;
        // C s_3_125: const #1s : i64
        let s_3_125: i64 = 1;
        // D s_3_126: add s_3_124 s_3_125
        let s_3_126: i64 = (s_3_124 + s_3_125);
        // D s_3_127: write-var p <= s_3_126
        fn_state.p = s_3_126;
        // N s_3_128: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#4309:i64
        let s_4_0: i64 = fn_state.VLshadow_4309;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var da:i64
        let s_4_3: i64 = fn_state.da;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var result:bv
        let s_4_6: Bits = fn_state.result;
        // D s_4_7: call Z_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = Z_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
