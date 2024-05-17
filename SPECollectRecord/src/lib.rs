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
use u_get_PMSLATFR_EL1_Type_MINLAT::*;
use HaveStatisticalProfilingv1p1::*;
use PMUEvent::*;
use u_get_PMSFCR_EL1_Type_LD::*;
use u_get_PMSFCR_EL1_Type_FnE::*;
use ConstrainUnpredictableBool::*;
use Bit::*;
use u_get_PMSFCR_EL1_Type_B::*;
use HaveTME::*;
use u_get_PMSFCR_EL1_Type_FDS::*;
use IsZero::*;
use u_get_PMSFCR_EL1_Type_FE::*;
use StatisticalProfilingEnabled::*;
use HaveSVE::*;
use HaveStatisticalProfilingv1p4::*;
use u__IMPDEF_bits::*;
use u_get_PMSFCR_EL1_Type_FT::*;
use HaveStatisticalProfilingv1p2::*;
use HaveStatisticalProfilingFDS::*;
use u_get_PMSFCR_EL1_Type_ST::*;
use u_get_PMSFCR_EL1_Type_FL::*;
use common::*;
pub fn SPECollectRecord<T: Tracer>(
    state: &mut State,
    tracer: &T,
    events: u64,
    total_latency: i128,
    optype: u32,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_25851: bool,
        gs_25882: bool,
        gs_25864: bool,
        gs_25865: bool,
        gs_25883: bool,
        is_op_stshadow_491: bool,
        mask: u64,
        gs_25881: bool,
        return_value: bool,
        gs_25863: bool,
        is_rejected_type: bool,
        gs_25859: bool,
        is_evt: bool,
        gs_25873: bool,
        gs_25874: bool,
        m: u64,
        e: u64,
        is_rejected_latency: bool,
        gs_25858: bool,
        gs_25872: bool,
        gs_25853: bool,
        mshadow_488: u64,
        is_op_ldshadow_490: bool,
        is_op_brshadow_489: bool,
        gs_25867: bool,
        is_nevt: bool,
        is_op: bool,
        is_rejected_event: bool,
        is_rejected_nevent: bool,
        gs_25866: bool,
        gs_25852: bool,
        is_lat: bool,
        events: u64,
        total_latency: i128,
        optype: u32,
    }
    let fn_state = FunctionState {
        events,
        total_latency,
        optype,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call StatisticalProfilingEnabled(s_0_0)
        let s_0_1: bool = StatisticalProfilingEnabled(state, tracer, s_0_0);
        // N s_0_2: assert s_0_1
        let s_0_2: () = assert!(s_0_1);
        // C s_0_3: const #64s : i
        let s_0_3: i128 = 64;
        // C s_0_4: const #170u : u8
        let s_0_4: u8 = 170;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 8u16);
        // D s_0_6: bits-cast zx s_0_5 -> bv length s_0_3
        let s_0_6: Bits = s_0_5.zero_extend(s_0_3);
        // D s_0_7: cast reint s_0_6 -> u64
        let s_0_7: u64 = (s_0_6.value() as u64);
        // D s_0_8: write-var mask <= s_0_7
        fn_state.mask = s_0_7;
        // C s_0_9: const #() : ()
        let s_0_9: () = ();
        // S s_0_10: call HaveSVE(s_0_9)
        let s_0_10: bool = HaveSVE(state, tracer, s_0_9);
        // N s_0_11: branch s_0_10 b113 b1
        if s_0_10 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call HaveTME(s_2_0)
        let s_2_1: bool = HaveTME(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b112 b3
        if s_2_1 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call HaveStatisticalProfilingv1p1(s_4_0)
        let s_4_1: bool = HaveStatisticalProfilingv1p1(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b111 b5
        if s_4_1 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HaveStatisticalProfilingv1p2(s_6_0)
        let s_6_1: bool = HaveStatisticalProfilingv1p2(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b110 b7
        if s_6_1 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HaveStatisticalProfilingv1p4(s_8_0)
        let s_8_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b109 b9
        if s_8_1 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #5s : i64
        let s_9_0: i64 = 5;
        // C s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // C s_9_2: const #"SPE mask 10:8,4,2" : str
        let s_9_2: &'static str = "SPE mask 10:8,4,2";
        // S s_9_3: call __IMPDEF_bits(s_9_1, s_9_2)
        let s_9_3: Bits = u__IMPDEF_bits(state, tracer, s_9_1, s_9_2);
        // S s_9_4: cast reint s_9_3 -> u8
        let s_9_4: u8 = (s_9_3.value() as u8);
        // C s_9_5: const #2s : i
        let s_9_5: i128 = 2;
        // S s_9_6: cast zx s_9_4 -> bv
        let s_9_6: Bits = Bits::new(s_9_4 as u128, 5u16);
        // C s_9_7: const #1s : i64
        let s_9_7: i64 = 1;
        // C s_9_8: cast zx s_9_7 -> i
        let s_9_8: i128 = (i128::try_from(s_9_7).unwrap());
        // C s_9_9: const #2s : i
        let s_9_9: i128 = 2;
        // C s_9_10: add s_9_9 s_9_8
        let s_9_10: i128 = (s_9_9 + s_9_8);
        // D s_9_11: bit-extract s_9_6 s_9_5 s_9_10
        let s_9_11: Bits = (Bits::new(
            ((s_9_6) >> (s_9_5)).value(),
            u16::try_from(s_9_10).unwrap(),
        ));
        // D s_9_12: cast reint s_9_11 -> u8
        let s_9_12: u8 = (s_9_11.value() as u8);
        // C s_9_13: const #8s : i
        let s_9_13: i128 = 8;
        // D s_9_14: read-var mask:u64
        let s_9_14: u64 = fn_state.mask;
        // D s_9_15: cast zx s_9_14 -> bv
        let s_9_15: Bits = Bits::new(s_9_14 as u128, 64u16);
        // D s_9_16: cast zx s_9_12 -> bv
        let s_9_16: Bits = Bits::new(s_9_12 as u128, 3u16);
        // C s_9_17: const #2s : i
        let s_9_17: i128 = 2;
        // C s_9_18: const #1u : u64
        let s_9_18: u64 = 1;
        // C s_9_19: cast zx s_9_18 -> bv
        let s_9_19: Bits = Bits::new(s_9_18 as u128, 64u16);
        // C s_9_20: lsl s_9_19 s_9_17
        let s_9_20: Bits = s_9_19 << s_9_17;
        // C s_9_21: sub s_9_20 s_9_19
        let s_9_21: Bits = ((s_9_20) - (s_9_19));
        // D s_9_22: and s_9_16 s_9_21
        let s_9_22: Bits = ((s_9_16) & (s_9_21));
        // D s_9_23: lsl s_9_22 s_9_13
        let s_9_23: Bits = s_9_22 << s_9_13;
        // C s_9_24: lsl s_9_21 s_9_13
        let s_9_24: Bits = s_9_21 << s_9_13;
        // C s_9_25: cmpl s_9_24
        let s_9_25: Bits = !s_9_24;
        // D s_9_26: and s_9_15 s_9_25
        let s_9_26: Bits = ((s_9_15) & (s_9_25));
        // D s_9_27: or s_9_26 s_9_23
        let s_9_27: Bits = ((s_9_26) | (s_9_23));
        // D s_9_28: cast reint s_9_27 -> u64
        let s_9_28: u64 = (s_9_27.value() as u64);
        // D s_9_29: write-var mask <= s_9_28
        fn_state.mask = s_9_28;
        // C s_9_30: const #1s : i
        let s_9_30: i128 = 1;
        // S s_9_31: cast zx s_9_4 -> bv
        let s_9_31: Bits = Bits::new(s_9_4 as u128, 5u16);
        // C s_9_32: const #1s : i64
        let s_9_32: i64 = 1;
        // C s_9_33: cast zx s_9_32 -> i
        let s_9_33: i128 = (i128::try_from(s_9_32).unwrap());
        // C s_9_34: const #0s : i
        let s_9_34: i128 = 0;
        // C s_9_35: add s_9_34 s_9_33
        let s_9_35: i128 = (s_9_34 + s_9_33);
        // D s_9_36: bit-extract s_9_31 s_9_30 s_9_35
        let s_9_36: Bits = (Bits::new(
            ((s_9_31) >> (s_9_30)).value(),
            u16::try_from(s_9_35).unwrap(),
        ));
        // D s_9_37: cast reint s_9_36 -> u8
        let s_9_37: bool = ((s_9_36.value()) != 0);
        // C s_9_38: const #4s : i
        let s_9_38: i128 = 4;
        // D s_9_39: read-var mask:u64
        let s_9_39: u64 = fn_state.mask;
        // D s_9_40: cast zx s_9_39 -> bv
        let s_9_40: Bits = Bits::new(s_9_39 as u128, 64u16);
        // D s_9_41: cast zx s_9_37 -> bv
        let s_9_41: Bits = Bits::new(s_9_37 as u128, 1u16);
        // C s_9_42: const #0s : i
        let s_9_42: i128 = 0;
        // C s_9_43: const #1u : u64
        let s_9_43: u64 = 1;
        // C s_9_44: cast zx s_9_43 -> bv
        let s_9_44: Bits = Bits::new(s_9_43 as u128, 64u16);
        // C s_9_45: lsl s_9_44 s_9_42
        let s_9_45: Bits = s_9_44 << s_9_42;
        // C s_9_46: sub s_9_45 s_9_44
        let s_9_46: Bits = ((s_9_45) - (s_9_44));
        // D s_9_47: and s_9_41 s_9_46
        let s_9_47: Bits = ((s_9_41) & (s_9_46));
        // D s_9_48: lsl s_9_47 s_9_38
        let s_9_48: Bits = s_9_47 << s_9_38;
        // C s_9_49: lsl s_9_46 s_9_38
        let s_9_49: Bits = s_9_46 << s_9_38;
        // C s_9_50: cmpl s_9_49
        let s_9_50: Bits = !s_9_49;
        // D s_9_51: and s_9_40 s_9_50
        let s_9_51: Bits = ((s_9_40) & (s_9_50));
        // D s_9_52: or s_9_51 s_9_48
        let s_9_52: Bits = ((s_9_51) | (s_9_48));
        // D s_9_53: cast reint s_9_52 -> u64
        let s_9_53: u64 = (s_9_52.value() as u64);
        // D s_9_54: write-var mask <= s_9_53
        fn_state.mask = s_9_53;
        // C s_9_55: const #0s : i
        let s_9_55: i128 = 0;
        // S s_9_56: cast zx s_9_4 -> bv
        let s_9_56: Bits = Bits::new(s_9_4 as u128, 5u16);
        // C s_9_57: const #1s : i64
        let s_9_57: i64 = 1;
        // C s_9_58: cast zx s_9_57 -> i
        let s_9_58: i128 = (i128::try_from(s_9_57).unwrap());
        // C s_9_59: const #0s : i
        let s_9_59: i128 = 0;
        // C s_9_60: add s_9_59 s_9_58
        let s_9_60: i128 = (s_9_59 + s_9_58);
        // D s_9_61: bit-extract s_9_56 s_9_55 s_9_60
        let s_9_61: Bits = (Bits::new(
            ((s_9_56) >> (s_9_55)).value(),
            u16::try_from(s_9_60).unwrap(),
        ));
        // D s_9_62: cast reint s_9_61 -> u8
        let s_9_62: bool = ((s_9_61.value()) != 0);
        // C s_9_63: const #2s : i
        let s_9_63: i128 = 2;
        // D s_9_64: read-var mask:u64
        let s_9_64: u64 = fn_state.mask;
        // D s_9_65: cast zx s_9_64 -> bv
        let s_9_65: Bits = Bits::new(s_9_64 as u128, 64u16);
        // D s_9_66: cast zx s_9_62 -> bv
        let s_9_66: Bits = Bits::new(s_9_62 as u128, 1u16);
        // C s_9_67: const #0s : i
        let s_9_67: i128 = 0;
        // C s_9_68: const #1u : u64
        let s_9_68: u64 = 1;
        // C s_9_69: cast zx s_9_68 -> bv
        let s_9_69: Bits = Bits::new(s_9_68 as u128, 64u16);
        // C s_9_70: lsl s_9_69 s_9_67
        let s_9_70: Bits = s_9_69 << s_9_67;
        // C s_9_71: sub s_9_70 s_9_69
        let s_9_71: Bits = ((s_9_70) - (s_9_69));
        // D s_9_72: and s_9_66 s_9_71
        let s_9_72: Bits = ((s_9_66) & (s_9_71));
        // D s_9_73: lsl s_9_72 s_9_63
        let s_9_73: Bits = s_9_72 << s_9_63;
        // C s_9_74: lsl s_9_71 s_9_63
        let s_9_74: Bits = s_9_71 << s_9_63;
        // C s_9_75: cmpl s_9_74
        let s_9_75: Bits = !s_9_74;
        // D s_9_76: and s_9_65 s_9_75
        let s_9_76: Bits = ((s_9_65) & (s_9_75));
        // D s_9_77: or s_9_76 s_9_73
        let s_9_77: Bits = ((s_9_76) | (s_9_73));
        // D s_9_78: cast reint s_9_77 -> u64
        let s_9_78: u64 = (s_9_77.value() as u64);
        // D s_9_79: write-var mask <= s_9_78
        fn_state.mask = s_9_78;
        // N s_9_80: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #16s : i64
        let s_10_0: i64 = 16;
        // C s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // C s_10_2: const #"SPE mask 63:48" : str
        let s_10_2: &'static str = "SPE mask 63:48";
        // S s_10_3: call __IMPDEF_bits(s_10_1, s_10_2)
        let s_10_3: Bits = u__IMPDEF_bits(state, tracer, s_10_1, s_10_2);
        // S s_10_4: cast reint s_10_3 -> u16
        let s_10_4: u16 = (s_10_3.value() as u16);
        // C s_10_5: const #48s : i
        let s_10_5: i128 = 48;
        // D s_10_6: read-var mask:u64
        let s_10_6: u64 = fn_state.mask;
        // D s_10_7: cast zx s_10_6 -> bv
        let s_10_7: Bits = Bits::new(s_10_6 as u128, 64u16);
        // S s_10_8: cast zx s_10_4 -> bv
        let s_10_8: Bits = Bits::new(s_10_4 as u128, 16u16);
        // C s_10_9: const #15s : i
        let s_10_9: i128 = 15;
        // C s_10_10: const #1u : u64
        let s_10_10: u64 = 1;
        // C s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 64u16);
        // C s_10_12: lsl s_10_11 s_10_9
        let s_10_12: Bits = s_10_11 << s_10_9;
        // C s_10_13: sub s_10_12 s_10_11
        let s_10_13: Bits = ((s_10_12) - (s_10_11));
        // S s_10_14: and s_10_8 s_10_13
        let s_10_14: Bits = ((s_10_8) & (s_10_13));
        // S s_10_15: lsl s_10_14 s_10_5
        let s_10_15: Bits = s_10_14 << s_10_5;
        // C s_10_16: lsl s_10_13 s_10_5
        let s_10_16: Bits = s_10_13 << s_10_5;
        // C s_10_17: cmpl s_10_16
        let s_10_17: Bits = !s_10_16;
        // D s_10_18: and s_10_7 s_10_17
        let s_10_18: Bits = ((s_10_7) & (s_10_17));
        // D s_10_19: or s_10_18 s_10_15
        let s_10_19: Bits = ((s_10_18) | (s_10_15));
        // D s_10_20: cast reint s_10_19 -> u64
        let s_10_20: u64 = (s_10_19.value() as u64);
        // D s_10_21: write-var mask <= s_10_20
        fn_state.mask = s_10_20;
        // C s_10_22: const #8s : i64
        let s_10_22: i64 = 8;
        // C s_10_23: cast zx s_10_22 -> i
        let s_10_23: i128 = (i128::try_from(s_10_22).unwrap());
        // C s_10_24: const #"SPE mask 31:24" : str
        let s_10_24: &'static str = "SPE mask 31:24";
        // S s_10_25: call __IMPDEF_bits(s_10_23, s_10_24)
        let s_10_25: Bits = u__IMPDEF_bits(state, tracer, s_10_23, s_10_24);
        // S s_10_26: cast reint s_10_25 -> u8
        let s_10_26: u8 = (s_10_25.value() as u8);
        // C s_10_27: const #24s : i
        let s_10_27: i128 = 24;
        // D s_10_28: read-var mask:u64
        let s_10_28: u64 = fn_state.mask;
        // D s_10_29: cast zx s_10_28 -> bv
        let s_10_29: Bits = Bits::new(s_10_28 as u128, 64u16);
        // S s_10_30: cast zx s_10_26 -> bv
        let s_10_30: Bits = Bits::new(s_10_26 as u128, 8u16);
        // C s_10_31: const #7s : i
        let s_10_31: i128 = 7;
        // C s_10_32: const #1u : u64
        let s_10_32: u64 = 1;
        // C s_10_33: cast zx s_10_32 -> bv
        let s_10_33: Bits = Bits::new(s_10_32 as u128, 64u16);
        // C s_10_34: lsl s_10_33 s_10_31
        let s_10_34: Bits = s_10_33 << s_10_31;
        // C s_10_35: sub s_10_34 s_10_33
        let s_10_35: Bits = ((s_10_34) - (s_10_33));
        // S s_10_36: and s_10_30 s_10_35
        let s_10_36: Bits = ((s_10_30) & (s_10_35));
        // S s_10_37: lsl s_10_36 s_10_27
        let s_10_37: Bits = s_10_36 << s_10_27;
        // C s_10_38: lsl s_10_35 s_10_27
        let s_10_38: Bits = s_10_35 << s_10_27;
        // C s_10_39: cmpl s_10_38
        let s_10_39: Bits = !s_10_38;
        // D s_10_40: and s_10_29 s_10_39
        let s_10_40: Bits = ((s_10_29) & (s_10_39));
        // D s_10_41: or s_10_40 s_10_37
        let s_10_41: Bits = ((s_10_40) | (s_10_37));
        // D s_10_42: cast reint s_10_41 -> u64
        let s_10_42: u64 = (s_10_41.value() as u64);
        // D s_10_43: write-var mask <= s_10_42
        fn_state.mask = s_10_42;
        // C s_10_44: const #4s : i64
        let s_10_44: i64 = 4;
        // C s_10_45: cast zx s_10_44 -> i
        let s_10_45: i128 = (i128::try_from(s_10_44).unwrap());
        // C s_10_46: const #"SPE mask 15:12" : str
        let s_10_46: &'static str = "SPE mask 15:12";
        // S s_10_47: call __IMPDEF_bits(s_10_45, s_10_46)
        let s_10_47: Bits = u__IMPDEF_bits(state, tracer, s_10_45, s_10_46);
        // S s_10_48: cast reint s_10_47 -> u8
        let s_10_48: u8 = (s_10_47.value() as u8);
        // C s_10_49: const #12s : i
        let s_10_49: i128 = 12;
        // D s_10_50: read-var mask:u64
        let s_10_50: u64 = fn_state.mask;
        // D s_10_51: cast zx s_10_50 -> bv
        let s_10_51: Bits = Bits::new(s_10_50 as u128, 64u16);
        // S s_10_52: cast zx s_10_48 -> bv
        let s_10_52: Bits = Bits::new(s_10_48 as u128, 4u16);
        // C s_10_53: const #3s : i
        let s_10_53: i128 = 3;
        // C s_10_54: const #1u : u64
        let s_10_54: u64 = 1;
        // C s_10_55: cast zx s_10_54 -> bv
        let s_10_55: Bits = Bits::new(s_10_54 as u128, 64u16);
        // C s_10_56: lsl s_10_55 s_10_53
        let s_10_56: Bits = s_10_55 << s_10_53;
        // C s_10_57: sub s_10_56 s_10_55
        let s_10_57: Bits = ((s_10_56) - (s_10_55));
        // S s_10_58: and s_10_52 s_10_57
        let s_10_58: Bits = ((s_10_52) & (s_10_57));
        // S s_10_59: lsl s_10_58 s_10_49
        let s_10_59: Bits = s_10_58 << s_10_49;
        // C s_10_60: lsl s_10_57 s_10_49
        let s_10_60: Bits = s_10_57 << s_10_49;
        // C s_10_61: cmpl s_10_60
        let s_10_61: Bits = !s_10_60;
        // D s_10_62: and s_10_51 s_10_61
        let s_10_62: Bits = ((s_10_51) & (s_10_61));
        // D s_10_63: or s_10_62 s_10_59
        let s_10_63: Bits = ((s_10_62) | (s_10_59));
        // D s_10_64: cast reint s_10_63 -> u64
        let s_10_64: u64 = (s_10_63.value() as u64);
        // D s_10_65: write-var mask <= s_10_64
        fn_state.mask = s_10_64;
        // D s_10_66: read-var events:u64
        let s_10_66: u64 = fn_state.events;
        // D s_10_67: cast zx s_10_66 -> bv
        let s_10_67: Bits = Bits::new(s_10_66 as u128, 64u16);
        // D s_10_68: read-var mask:u64
        let s_10_68: u64 = fn_state.mask;
        // D s_10_69: cast zx s_10_68 -> bv
        let s_10_69: Bits = Bits::new(s_10_68 as u128, 64u16);
        // D s_10_70: and s_10_67 s_10_69
        let s_10_70: Bits = ((s_10_67) & (s_10_69));
        // D s_10_71: cast reint s_10_70 -> u64
        let s_10_71: u64 = (s_10_70.value() as u64);
        // D s_10_72: write-var e <= s_10_71
        fn_state.e = s_10_71;
        // C s_10_73: const #0u : u8
        let s_10_73: bool = false;
        // D s_10_74: write-var is_rejected_nevent <= s_10_73
        fn_state.is_rejected_nevent = s_10_73;
        // C s_10_75: const #() : ()
        let s_10_75: () = ();
        // S s_10_76: call HaveStatisticalProfilingv1p2(s_10_75)
        let s_10_76: bool = HaveStatisticalProfilingv1p2(state, tracer, s_10_75);
        // N s_10_77: branch s_10_76 b103 b11
        if s_10_76 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var is_nevt <= s_11_0
        fn_state.is_nevt = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var is_rejected_event <= s_12_0
        fn_state.is_rejected_event = s_12_0;
        // C s_12_2: const #22744u : u32
        let s_12_2: u32 = 22744;
        // D s_12_3: read-reg s_12_2:u64
        let s_12_3: u64 = {
            let value = state.read_register::<u64>(s_12_2 as isize);
            tracer.read_register(s_12_2 as isize, value);
            value
        };
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 64u16);
        // D s_12_5: read-var mask:u64
        let s_12_5: u64 = fn_state.mask;
        // D s_12_6: cast zx s_12_5 -> bv
        let s_12_6: Bits = Bits::new(s_12_5 as u128, 64u16);
        // D s_12_7: and s_12_4 s_12_6
        let s_12_7: Bits = ((s_12_4) & (s_12_6));
        // D s_12_8: cast reint s_12_7 -> u64
        let s_12_8: u64 = (s_12_7.value() as u64);
        // D s_12_9: write-var mshadow#488 <= s_12_8
        fn_state.mshadow_488 = s_12_8;
        // D s_12_10: read-var e:u64
        let s_12_10: u64 = fn_state.e;
        // D s_12_11: cast zx s_12_10 -> bv
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 64u16);
        // D s_12_12: not s_12_11
        let s_12_12: Bits = !s_12_11;
        // D s_12_13: cast reint s_12_12 -> u64
        let s_12_13: u64 = (s_12_12.value() as u64);
        // D s_12_14: cast zx s_12_13 -> bv
        let s_12_14: Bits = Bits::new(s_12_13 as u128, 64u16);
        // D s_12_15: read-var mshadow#488:u64
        let s_12_15: u64 = fn_state.mshadow_488;
        // D s_12_16: cast zx s_12_15 -> bv
        let s_12_16: Bits = Bits::new(s_12_15 as u128, 64u16);
        // D s_12_17: and s_12_14 s_12_16
        let s_12_17: Bits = ((s_12_14) & (s_12_16));
        // D s_12_18: cast reint s_12_17 -> u64
        let s_12_18: u64 = (s_12_17.value() as u64);
        // D s_12_19: cast zx s_12_18 -> bv
        let s_12_19: Bits = Bits::new(s_12_18 as u128, 64u16);
        // D s_12_20: call IsZero(s_12_19)
        let s_12_20: bool = IsZero(state, tracer, s_12_19);
        // D s_12_21: write-var is_evt <= s_12_20
        fn_state.is_evt = s_12_20;
        // C s_12_22: const #101208u : u32
        let s_12_22: u32 = 101208;
        // D s_12_23: read-reg s_12_22:struct
        let s_12_23: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_22 as isize);
            tracer.read_register(s_12_22 as isize, value);
            value
        };
        // D s_12_24: call _get_PMSFCR_EL1_Type_FE(s_12_23)
        let s_12_24: bool = u_get_PMSFCR_EL1_Type_FE(state, tracer, s_12_23);
        // D s_12_25: cast zx s_12_24 -> bv
        let s_12_25: Bits = Bits::new(s_12_24 as u128, 1u16);
        // C s_12_26: const #1u : u8
        let s_12_26: bool = true;
        // C s_12_27: cast zx s_12_26 -> bv
        let s_12_27: Bits = Bits::new(s_12_26 as u128, 1u16);
        // D s_12_28: cmp-eq s_12_25 s_12_27
        let s_12_28: bool = ((s_12_25) == (s_12_27));
        // N s_12_29: branch s_12_28 b100 b13
        if s_12_28 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaveStatisticalProfilingv1p2(s_14_0)
        let s_14_1: bool = HaveStatisticalProfilingv1p2(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b99 b15
        if s_14_1 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#25851 <= s_15_0
        fn_state.gs_25851 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var gs#25851:u8
        let s_16_0: bool = fn_state.gs_25851;
        // N s_16_1: branch s_16_0 b98 b17
        if s_16_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#25852 <= s_17_0
        fn_state.gs_25852 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#25852:u8
        let s_18_0: bool = fn_state.gs_25852;
        // N s_18_1: branch s_18_0 b97 b19
        if s_18_0 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var is_evt:u8
        let s_20_0: bool = fn_state.is_evt;
        // N s_20_1: branch s_20_0 b96 b21
        if s_20_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#25853 <= s_21_0
        fn_state.gs_25853 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var gs#25853:u8
        let s_22_0: bool = fn_state.gs_25853;
        // N s_22_1: branch s_22_0 b95 b23
        if s_22_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var optype:u32
        let s_24_0: u32 = fn_state.optype;
        // C s_24_1: const #3u : u32
        let s_24_1: u32 = 3;
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
        // D s_24_3: write-var is_op_brshadow#489 <= s_24_2
        fn_state.is_op_brshadow_489 = s_24_2;
        // D s_24_4: read-var optype:u32
        let s_24_4: u32 = fn_state.optype;
        // C s_24_5: const #0u : u32
        let s_24_5: u32 = 0;
        // D s_24_6: cmp-eq s_24_4 s_24_5
        let s_24_6: bool = ((s_24_4) == (s_24_5));
        // N s_24_7: branch s_24_6 b94 b25
        if s_24_6 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var optype:u32
        let s_25_0: u32 = fn_state.optype;
        // C s_25_1: const #2u : u32
        let s_25_1: u32 = 2;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // D s_25_3: write-var gs#25858 <= s_25_2
        fn_state.gs_25858 = s_25_2;
        // N s_25_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var gs#25858:u8
        let s_26_0: bool = fn_state.gs_25858;
        // D s_26_1: write-var is_op_ldshadow#490 <= s_26_0
        fn_state.is_op_ldshadow_490 = s_26_0;
        // D s_26_2: read-var optype:u32
        let s_26_2: u32 = fn_state.optype;
        // C s_26_3: const #1u : u32
        let s_26_3: u32 = 1;
        // D s_26_4: cmp-eq s_26_2 s_26_3
        let s_26_4: bool = ((s_26_2) == (s_26_3));
        // N s_26_5: branch s_26_4 b93 b27
        if s_26_4 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var optype:u32
        let s_27_0: u32 = fn_state.optype;
        // C s_27_1: const #2u : u32
        let s_27_1: u32 = 2;
        // D s_27_2: cmp-eq s_27_0 s_27_1
        let s_27_2: bool = ((s_27_0) == (s_27_1));
        // D s_27_3: write-var gs#25859 <= s_27_2
        fn_state.gs_25859 = s_27_2;
        // N s_27_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var gs#25859:u8
        let s_28_0: bool = fn_state.gs_25859;
        // D s_28_1: write-var is_op_stshadow#491 <= s_28_0
        fn_state.is_op_stshadow_491 = s_28_0;
        // D s_28_2: read-var is_op_brshadow#489:u8
        let s_28_2: bool = fn_state.is_op_brshadow_489;
        // N s_28_3: branch s_28_2 b92 b29
        if s_28_2 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var is_op_ldshadow#490:u8
        let s_30_0: bool = fn_state.is_op_ldshadow_490;
        // N s_30_1: branch s_30_0 b91 b31
        if s_30_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_32_0: read-var is_op_stshadow#491:u8
        let s_32_0: bool = fn_state.is_op_stshadow_491;
        // N s_32_1: branch s_32_0 b90 b33
        if s_32_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_34_0: read-var is_op_brshadow#489:u8
        let s_34_0: bool = fn_state.is_op_brshadow_489;
        // N s_34_1: branch s_34_0 b89 b35
        if s_34_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#25863 <= s_35_0
        fn_state.gs_25863 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_36_0: read-var gs#25863:u8
        let s_36_0: bool = fn_state.gs_25863;
        // N s_36_1: branch s_36_0 b88 b37
        if s_36_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_37_0: read-var is_op_ldshadow#490:u8
        let s_37_0: bool = fn_state.is_op_ldshadow_490;
        // N s_37_1: branch s_37_0 b87 b38
        if s_37_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#25864 <= s_38_0
        fn_state.gs_25864 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_39_0: read-var gs#25864:u8
        let s_39_0: bool = fn_state.gs_25864;
        // D s_39_1: write-var gs#25865 <= s_39_0
        fn_state.gs_25865 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_40_0: read-var gs#25865:u8
        let s_40_0: bool = fn_state.gs_25865;
        // N s_40_1: branch s_40_0 b86 b41
        if s_40_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_41_0: read-var is_op_stshadow#491:u8
        let s_41_0: bool = fn_state.is_op_stshadow_491;
        // N s_41_1: branch s_41_0 b85 b42
        if s_41_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#25866 <= s_42_0
        fn_state.gs_25866 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_43_0: read-var gs#25866:u8
        let s_43_0: bool = fn_state.gs_25866;
        // D s_43_1: write-var gs#25867 <= s_43_0
        fn_state.gs_25867 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_44_0: read-var gs#25867:u8
        let s_44_0: bool = fn_state.gs_25867;
        // D s_44_1: write-var is_op <= s_44_0
        fn_state.is_op = s_44_0;
        // D s_44_2: read-var is_op:u8
        let s_44_2: bool = fn_state.is_op;
        // N s_44_3: branch s_44_2 b84 b45
        if s_44_2 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var is_rejected_type <= s_46_0
        fn_state.is_rejected_type = s_46_0;
        // C s_46_2: const #101208u : u32
        let s_46_2: u32 = 101208;
        // D s_46_3: read-reg s_46_2:struct
        let s_46_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_46_2 as isize);
            tracer.read_register(s_46_2 as isize, value);
            value
        };
        // D s_46_4: call _get_PMSFCR_EL1_Type_FT(s_46_3)
        let s_46_4: bool = u_get_PMSFCR_EL1_Type_FT(state, tracer, s_46_3);
        // D s_46_5: cast zx s_46_4 -> bv
        let s_46_5: Bits = Bits::new(s_46_4 as u128, 1u16);
        // C s_46_6: const #1u : u8
        let s_46_6: bool = true;
        // C s_46_7: cast zx s_46_6 -> bv
        let s_46_7: Bits = Bits::new(s_46_6 as u128, 1u16);
        // D s_46_8: cmp-eq s_46_5 s_46_7
        let s_46_8: bool = ((s_46_5) == (s_46_7));
        // N s_46_9: branch s_46_8 b81 b47
        if s_46_8 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_47_0: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var is_rejected_latency <= s_48_0
        fn_state.is_rejected_latency = s_48_0;
        // C s_48_2: const #10144u : u32
        let s_48_2: u32 = 10144;
        // D s_48_3: read-reg s_48_2:struct
        let s_48_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_48_2 as isize);
            tracer.read_register(s_48_2 as isize, value);
            value
        };
        // D s_48_4: call _get_PMSLATFR_EL1_Type_MINLAT(s_48_3)
        let s_48_4: u16 = u_get_PMSLATFR_EL1_Type_MINLAT(state, tracer, s_48_3);
        // D s_48_5: cast zx s_48_4 -> bv
        let s_48_5: Bits = Bits::new(s_48_4 as u128, 16u16);
        // D s_48_6: cast zx s_48_5 -> i
        let s_48_6: i128 = (s_48_5.value() as i128);
        // D s_48_7: cast reint s_48_6 -> i64
        let s_48_7: i64 = (s_48_6 as i64);
        // D s_48_8: cast zx s_48_7 -> i
        let s_48_8: i128 = (i128::try_from(s_48_7).unwrap());
        // D s_48_9: read-var total_latency:i
        let s_48_9: i128 = fn_state.total_latency;
        // D s_48_10: cmp-lt s_48_9 s_48_8
        let s_48_10: bool = ((s_48_9) < (s_48_8));
        // D s_48_11: write-var is_lat <= s_48_10
        fn_state.is_lat = s_48_10;
        // D s_48_12: read-var is_lat:u8
        let s_48_12: bool = fn_state.is_lat;
        // N s_48_13: branch s_48_12 b80 b49
        if s_48_12 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_49_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_50_0: const #101208u : u32
        let s_50_0: u32 = 101208;
        // D s_50_1: read-reg s_50_0:struct
        let s_50_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call _get_PMSFCR_EL1_Type_FL(s_50_1)
        let s_50_2: bool = u_get_PMSFCR_EL1_Type_FL(state, tracer, s_50_1);
        // D s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // C s_50_4: const #1u : u8
        let s_50_4: bool = true;
        // C s_50_5: cast zx s_50_4 -> bv
        let s_50_5: Bits = Bits::new(s_50_4 as u128, 1u16);
        // D s_50_6: cmp-eq s_50_3 s_50_5
        let s_50_6: bool = ((s_50_3) == (s_50_5));
        // N s_50_7: branch s_50_6 b77 b51
        if s_50_6 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_51_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call HaveStatisticalProfilingFDS(s_52_0)
        let s_52_1: bool = HaveStatisticalProfilingFDS(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b76 b53
        if s_52_1 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#25872 <= s_53_0
        fn_state.gs_25872 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_54_0: read-var gs#25872:u8
        let s_54_0: bool = fn_state.gs_25872;
        // N s_54_1: branch s_54_0 b75 b55
        if s_54_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#25873 <= s_55_0
        fn_state.gs_25873 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_56_0: read-var gs#25873:u8
        let s_56_0: bool = fn_state.gs_25873;
        // N s_56_1: branch s_56_0 b74 b57
        if s_56_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#25874 <= s_57_0
        fn_state.gs_25874 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_58_0: read-var gs#25874:u8
        let s_58_0: bool = fn_state.gs_25874;
        // N s_58_1: branch s_58_0 b73 b59
        if s_58_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_59_0: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_60_0: read-var is_rejected_nevent:u8
        let s_60_0: bool = fn_state.is_rejected_nevent;
        // N s_60_1: branch s_60_0 b72 b61
        if s_60_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_61_0: read-var is_rejected_event:u8
        let s_61_0: bool = fn_state.is_rejected_event;
        // D s_61_1: write-var gs#25881 <= s_61_0
        fn_state.gs_25881 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_62_0: read-var gs#25881:u8
        let s_62_0: bool = fn_state.gs_25881;
        // N s_62_1: branch s_62_0 b71 b63
        if s_62_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_63_0: read-var is_rejected_type:u8
        let s_63_0: bool = fn_state.is_rejected_type;
        // D s_63_1: write-var gs#25882 <= s_63_0
        fn_state.gs_25882 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_64_0: read-var gs#25882:u8
        let s_64_0: bool = fn_state.gs_25882;
        // N s_64_1: branch s_64_0 b70 b65
        if s_64_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_65_0: read-var is_rejected_latency:u8
        let s_65_0: bool = fn_state.is_rejected_latency;
        // D s_65_1: write-var gs#25883 <= s_65_0
        fn_state.gs_25883 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_66_0: read-var gs#25883:u8
        let s_66_0: bool = fn_state.gs_25883;
        // D s_66_1: not s_66_0
        let s_66_1: bool = !s_66_0;
        // D s_66_2: write-var return_value <= s_66_1
        fn_state.return_value = s_66_1;
        // D s_66_3: read-var return_value:u8
        let s_66_3: bool = fn_state.return_value;
        // N s_66_4: branch s_66_3 b69 b67
        if s_66_3 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_67_0: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_68_0: read-var return_value:u8
        let s_68_0: bool = fn_state.return_value;
        // D s_68_1: write-var return_value <= s_68_0
        fn_state.return_value = s_68_0;
        // D s_68_2: read-var return_value:u8
        let s_68_2: bool = fn_state.return_value;
        // N s_68_3: return s_68_2
        return s_68_2;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_69_0: const #168u : u32
        let s_69_0: u32 = 168;
        // D s_69_1: read-reg s_69_0:u16
        let s_69_1: u16 = {
            let value = state.read_register::<u16>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: call PMUEvent(s_69_1)
        let s_69_2: () = PMUEvent(state, tracer, s_69_1);
        // N s_69_3: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#25883 <= s_70_0
        fn_state.gs_25883 = s_70_0;
        // N s_70_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#25882 <= s_71_0
        fn_state.gs_25882 = s_71_0;
        // N s_71_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#25881 <= s_72_0
        fn_state.gs_25881 = s_72_0;
        // N s_72_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_73_0: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_74_0: const #14184u : u32
        let s_74_0: u32 = 14184;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: bool = {
            let value = state.read_register::<bool>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: write-var gs#25874 <= s_74_1
        fn_state.gs_25874 = s_74_1;
        // N s_74_3: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_75_0: read-var is_op_ldshadow#490:u8
        let s_75_0: bool = fn_state.is_op_ldshadow_490;
        // D s_75_1: write-var gs#25873 <= s_75_0
        fn_state.gs_25873 = s_75_0;
        // N s_75_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_76_0: const #101208u : u32
        let s_76_0: u32 = 101208;
        // D s_76_1: read-reg s_76_0:struct
        let s_76_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // D s_76_2: call _get_PMSFCR_EL1_Type_FDS(s_76_1)
        let s_76_2: bool = u_get_PMSFCR_EL1_Type_FDS(state, tracer, s_76_1);
        // D s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // C s_76_4: const #1u : u8
        let s_76_4: bool = true;
        // C s_76_5: cast zx s_76_4 -> bv
        let s_76_5: Bits = Bits::new(s_76_4 as u128, 1u16);
        // D s_76_6: cmp-eq s_76_3 s_76_5
        let s_76_6: bool = ((s_76_3) == (s_76_5));
        // D s_76_7: write-var gs#25872 <= s_76_6
        fn_state.gs_25872 = s_76_6;
        // N s_76_8: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_77_0: const #10144u : u32
        let s_77_0: u32 = 10144;
        // D s_77_1: read-reg s_77_0:struct
        let s_77_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call _get_PMSLATFR_EL1_Type_MINLAT(s_77_1)
        let s_77_2: u16 = u_get_PMSLATFR_EL1_Type_MINLAT(state, tracer, s_77_1);
        // D s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 16u16);
        // D s_77_4: call IsZero(s_77_3)
        let s_77_4: bool = IsZero(state, tracer, s_77_3);
        // D s_77_5: not s_77_4
        let s_77_5: bool = !s_77_4;
        // N s_77_6: branch s_77_5 b79 b78
        if s_77_5 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_78_0: const #60u : u32
        let s_78_0: u32 = 60;
        // S s_78_1: call ConstrainUnpredictableBool(s_78_0)
        let s_78_1: bool = ConstrainUnpredictableBool(state, tracer, s_78_0);
        // D s_78_2: write-var is_rejected_latency <= s_78_1
        fn_state.is_rejected_latency = s_78_1;
        // N s_78_3: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_79_0: read-var is_lat:u8
        let s_79_0: bool = fn_state.is_lat;
        // D s_79_1: not s_79_0
        let s_79_1: bool = !s_79_0;
        // D s_79_2: write-var is_rejected_latency <= s_79_1
        fn_state.is_rejected_latency = s_79_1;
        // N s_79_3: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_80_0: const #272u : u32
        let s_80_0: u32 = 272;
        // D s_80_1: read-reg s_80_0:u16
        let s_80_1: u16 = {
            let value = state.read_register::<u16>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // D s_80_2: call PMUEvent(s_80_1)
        let s_80_2: () = PMUEvent(state, tracer, s_80_1);
        // N s_80_3: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_81_0: const #101208u : u32
        let s_81_0: u32 = 101208;
        // D s_81_1: read-reg s_81_0:struct
        let s_81_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call _get_PMSFCR_EL1_Type_B(s_81_1)
        let s_81_2: bool = u_get_PMSFCR_EL1_Type_B(state, tracer, s_81_1);
        // C s_81_3: const #101208u : u32
        let s_81_3: u32 = 101208;
        // D s_81_4: read-reg s_81_3:struct
        let s_81_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_81_3 as isize);
            tracer.read_register(s_81_3 as isize, value);
            value
        };
        // D s_81_5: call _get_PMSFCR_EL1_Type_LD(s_81_4)
        let s_81_5: bool = u_get_PMSFCR_EL1_Type_LD(state, tracer, s_81_4);
        // C s_81_6: const #101208u : u32
        let s_81_6: u32 = 101208;
        // D s_81_7: read-reg s_81_6:struct
        let s_81_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_81_6 as isize);
            tracer.read_register(s_81_6 as isize, value);
            value
        };
        // D s_81_8: call _get_PMSFCR_EL1_Type_ST(s_81_7)
        let s_81_8: bool = u_get_PMSFCR_EL1_Type_ST(state, tracer, s_81_7);
        // D s_81_9: cast zx s_81_5 -> bv
        let s_81_9: Bits = Bits::new(s_81_5 as u128, 1u16);
        // D s_81_10: cast zx s_81_8 -> bv
        let s_81_10: Bits = Bits::new(s_81_8 as u128, 1u16);
        // D s_81_11: cast reint s_81_9 -> u128
        let s_81_11: u128 = (s_81_9.value() as u128);
        // D s_81_12: size-of s_81_9
        let s_81_12: u16 = s_81_9.length();
        // D s_81_13: cast reint s_81_10 -> u128
        let s_81_13: u128 = (s_81_10.value() as u128);
        // D s_81_14: size-of s_81_10
        let s_81_14: u16 = s_81_10.length();
        // D s_81_15: lsl s_81_11 s_81_14
        let s_81_15: u128 = s_81_11 << s_81_14;
        // D s_81_16: or s_81_15 s_81_13
        let s_81_16: u128 = ((s_81_15) | (s_81_13));
        // D s_81_17: add s_81_12 s_81_14
        let s_81_17: u16 = (s_81_12 + s_81_14);
        // D s_81_18: create-bits s_81_16 s_81_17
        let s_81_18: Bits = Bits::new(s_81_16, s_81_17);
        // D s_81_19: cast reint s_81_18 -> u8
        let s_81_19: u8 = (s_81_18.value() as u8);
        // D s_81_20: cast zx s_81_2 -> bv
        let s_81_20: Bits = Bits::new(s_81_2 as u128, 1u16);
        // D s_81_21: cast zx s_81_19 -> bv
        let s_81_21: Bits = Bits::new(s_81_19 as u128, 2u16);
        // D s_81_22: cast reint s_81_20 -> u128
        let s_81_22: u128 = (s_81_20.value() as u128);
        // D s_81_23: size-of s_81_20
        let s_81_23: u16 = s_81_20.length();
        // D s_81_24: cast reint s_81_21 -> u128
        let s_81_24: u128 = (s_81_21.value() as u128);
        // D s_81_25: size-of s_81_21
        let s_81_25: u16 = s_81_21.length();
        // D s_81_26: lsl s_81_22 s_81_25
        let s_81_26: u128 = s_81_22 << s_81_25;
        // D s_81_27: or s_81_26 s_81_24
        let s_81_27: u128 = ((s_81_26) | (s_81_24));
        // D s_81_28: add s_81_23 s_81_25
        let s_81_28: u16 = (s_81_23 + s_81_25);
        // D s_81_29: create-bits s_81_27 s_81_28
        let s_81_29: Bits = Bits::new(s_81_27, s_81_28);
        // D s_81_30: cast reint s_81_29 -> u8
        let s_81_30: u8 = (s_81_29.value() as u8);
        // D s_81_31: cast zx s_81_30 -> bv
        let s_81_31: Bits = Bits::new(s_81_30 as u128, 3u16);
        // D s_81_32: call IsZero(s_81_31)
        let s_81_32: bool = IsZero(state, tracer, s_81_31);
        // D s_81_33: not s_81_32
        let s_81_33: bool = !s_81_32;
        // N s_81_34: branch s_81_33 b83 b82
        if s_81_33 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_82_0: const #60u : u32
        let s_82_0: u32 = 60;
        // S s_82_1: call ConstrainUnpredictableBool(s_82_0)
        let s_82_1: bool = ConstrainUnpredictableBool(state, tracer, s_82_0);
        // D s_82_2: write-var is_rejected_type <= s_82_1
        fn_state.is_rejected_type = s_82_1;
        // N s_82_3: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_83_0: read-var is_op:u8
        let s_83_0: bool = fn_state.is_op;
        // D s_83_1: not s_83_0
        let s_83_1: bool = !s_83_0;
        // D s_83_2: write-var is_rejected_type <= s_83_1
        fn_state.is_rejected_type = s_83_1;
        // N s_83_3: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_84_0: const #256u : u32
        let s_84_0: u32 = 256;
        // D s_84_1: read-reg s_84_0:u16
        let s_84_1: u16 = {
            let value = state.read_register::<u16>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call PMUEvent(s_84_1)
        let s_84_2: () = PMUEvent(state, tracer, s_84_1);
        // N s_84_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_85_0: const #101208u : u32
        let s_85_0: u32 = 101208;
        // D s_85_1: read-reg s_85_0:struct
        let s_85_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call _get_PMSFCR_EL1_Type_ST(s_85_1)
        let s_85_2: bool = u_get_PMSFCR_EL1_Type_ST(state, tracer, s_85_1);
        // D s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // C s_85_4: const #1u : u8
        let s_85_4: bool = true;
        // C s_85_5: cast zx s_85_4 -> bv
        let s_85_5: Bits = Bits::new(s_85_4 as u128, 1u16);
        // D s_85_6: cmp-eq s_85_3 s_85_5
        let s_85_6: bool = ((s_85_3) == (s_85_5));
        // D s_85_7: write-var gs#25866 <= s_85_6
        fn_state.gs_25866 = s_85_6;
        // N s_85_8: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_86_0: const #1u : u8
        let s_86_0: bool = true;
        // D s_86_1: write-var gs#25867 <= s_86_0
        fn_state.gs_25867 = s_86_0;
        // N s_86_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_87_0: const #101208u : u32
        let s_87_0: u32 = 101208;
        // D s_87_1: read-reg s_87_0:struct
        let s_87_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_87_0 as isize);
            tracer.read_register(s_87_0 as isize, value);
            value
        };
        // D s_87_2: call _get_PMSFCR_EL1_Type_LD(s_87_1)
        let s_87_2: bool = u_get_PMSFCR_EL1_Type_LD(state, tracer, s_87_1);
        // D s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 1u16);
        // C s_87_4: const #1u : u8
        let s_87_4: bool = true;
        // C s_87_5: cast zx s_87_4 -> bv
        let s_87_5: Bits = Bits::new(s_87_4 as u128, 1u16);
        // D s_87_6: cmp-eq s_87_3 s_87_5
        let s_87_6: bool = ((s_87_3) == (s_87_5));
        // D s_87_7: write-var gs#25864 <= s_87_6
        fn_state.gs_25864 = s_87_6;
        // N s_87_8: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_88_0: const #1u : u8
        let s_88_0: bool = true;
        // D s_88_1: write-var gs#25865 <= s_88_0
        fn_state.gs_25865 = s_88_0;
        // N s_88_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_89_0: const #101208u : u32
        let s_89_0: u32 = 101208;
        // D s_89_1: read-reg s_89_0:struct
        let s_89_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // D s_89_2: call _get_PMSFCR_EL1_Type_B(s_89_1)
        let s_89_2: bool = u_get_PMSFCR_EL1_Type_B(state, tracer, s_89_1);
        // D s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // C s_89_4: const #1u : u8
        let s_89_4: bool = true;
        // C s_89_5: cast zx s_89_4 -> bv
        let s_89_5: Bits = Bits::new(s_89_4 as u128, 1u16);
        // D s_89_6: cmp-eq s_89_3 s_89_5
        let s_89_6: bool = ((s_89_3) == (s_89_5));
        // D s_89_7: write-var gs#25863 <= s_89_6
        fn_state.gs_25863 = s_89_6;
        // N s_89_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_90_0: const #248u : u32
        let s_90_0: u32 = 248;
        // D s_90_1: read-reg s_90_0:u16
        let s_90_1: u16 = {
            let value = state.read_register::<u16>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // D s_90_2: call PMUEvent(s_90_1)
        let s_90_2: () = PMUEvent(state, tracer, s_90_1);
        // N s_90_3: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_91_0: const #240u : u32
        let s_91_0: u32 = 240;
        // D s_91_1: read-reg s_91_0:u16
        let s_91_1: u16 = {
            let value = state.read_register::<u16>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // D s_91_2: call PMUEvent(s_91_1)
        let s_91_2: () = PMUEvent(state, tracer, s_91_1);
        // N s_91_3: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_92_0: const #232u : u32
        let s_92_0: u32 = 232;
        // D s_92_1: read-reg s_92_0:u16
        let s_92_1: u16 = {
            let value = state.read_register::<u16>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // D s_92_2: call PMUEvent(s_92_1)
        let s_92_2: () = PMUEvent(state, tracer, s_92_1);
        // N s_92_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_93_0: const #1u : u8
        let s_93_0: bool = true;
        // D s_93_1: write-var gs#25859 <= s_93_0
        fn_state.gs_25859 = s_93_0;
        // N s_93_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_94_0: const #1u : u8
        let s_94_0: bool = true;
        // D s_94_1: write-var gs#25858 <= s_94_0
        fn_state.gs_25858 = s_94_0;
        // N s_94_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_95_0: const #264u : u32
        let s_95_0: u32 = 264;
        // D s_95_1: read-reg s_95_0:u16
        let s_95_1: u16 = {
            let value = state.read_register::<u16>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // D s_95_2: call PMUEvent(s_95_1)
        let s_95_2: () = PMUEvent(state, tracer, s_95_1);
        // N s_95_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_96_0: read-var is_nevt:u8
        let s_96_0: bool = fn_state.is_nevt;
        // D s_96_1: write-var gs#25853 <= s_96_0
        fn_state.gs_25853 = s_96_0;
        // N s_96_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_97_0: const #60u : u32
        let s_97_0: u32 = 60;
        // S s_97_1: call ConstrainUnpredictableBool(s_97_0)
        let s_97_1: bool = ConstrainUnpredictableBool(state, tracer, s_97_0);
        // D s_97_2: write-var is_rejected_nevent <= s_97_1
        fn_state.is_rejected_nevent = s_97_1;
        // C s_97_3: const #60u : u32
        let s_97_3: u32 = 60;
        // S s_97_4: call ConstrainUnpredictableBool(s_97_3)
        let s_97_4: bool = ConstrainUnpredictableBool(state, tracer, s_97_3);
        // D s_97_5: write-var is_rejected_event <= s_97_4
        fn_state.is_rejected_event = s_97_4;
        // N s_97_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_98_0: const #22744u : u32
        let s_98_0: u32 = 22744;
        // D s_98_1: read-reg s_98_0:u64
        let s_98_1: u64 = {
            let value = state.read_register::<u64>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // C s_98_2: const #12904u : u32
        let s_98_2: u32 = 12904;
        // D s_98_3: read-reg s_98_2:u64
        let s_98_3: u64 = {
            let value = state.read_register::<u64>(s_98_2 as isize);
            tracer.read_register(s_98_2 as isize, value);
            value
        };
        // D s_98_4: cast zx s_98_1 -> bv
        let s_98_4: Bits = Bits::new(s_98_1 as u128, 64u16);
        // D s_98_5: cast zx s_98_3 -> bv
        let s_98_5: Bits = Bits::new(s_98_3 as u128, 64u16);
        // D s_98_6: and s_98_4 s_98_5
        let s_98_6: Bits = ((s_98_4) & (s_98_5));
        // D s_98_7: cast reint s_98_6 -> u64
        let s_98_7: u64 = (s_98_6.value() as u64);
        // D s_98_8: cast zx s_98_7 -> bv
        let s_98_8: Bits = Bits::new(s_98_7 as u128, 64u16);
        // D s_98_9: read-var mask:u64
        let s_98_9: u64 = fn_state.mask;
        // D s_98_10: cast zx s_98_9 -> bv
        let s_98_10: Bits = Bits::new(s_98_9 as u128, 64u16);
        // D s_98_11: and s_98_8 s_98_10
        let s_98_11: Bits = ((s_98_8) & (s_98_10));
        // D s_98_12: cast reint s_98_11 -> u64
        let s_98_12: u64 = (s_98_11.value() as u64);
        // D s_98_13: cast zx s_98_12 -> bv
        let s_98_13: Bits = Bits::new(s_98_12 as u128, 64u16);
        // D s_98_14: call IsZero(s_98_13)
        let s_98_14: bool = IsZero(state, tracer, s_98_13);
        // D s_98_15: not s_98_14
        let s_98_15: bool = !s_98_14;
        // D s_98_16: write-var gs#25852 <= s_98_15
        fn_state.gs_25852 = s_98_15;
        // N s_98_17: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_99_0: const #101208u : u32
        let s_99_0: u32 = 101208;
        // D s_99_1: read-reg s_99_0:struct
        let s_99_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // D s_99_2: call _get_PMSFCR_EL1_Type_FnE(s_99_1)
        let s_99_2: bool = u_get_PMSFCR_EL1_Type_FnE(state, tracer, s_99_1);
        // C s_99_3: const #101208u : u32
        let s_99_3: u32 = 101208;
        // D s_99_4: read-reg s_99_3:struct
        let s_99_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_99_3 as isize);
            tracer.read_register(s_99_3 as isize, value);
            value
        };
        // D s_99_5: call _get_PMSFCR_EL1_Type_FE(s_99_4)
        let s_99_5: bool = u_get_PMSFCR_EL1_Type_FE(state, tracer, s_99_4);
        // D s_99_6: cast zx s_99_2 -> bv
        let s_99_6: Bits = Bits::new(s_99_2 as u128, 1u16);
        // D s_99_7: cast zx s_99_5 -> bv
        let s_99_7: Bits = Bits::new(s_99_5 as u128, 1u16);
        // D s_99_8: cast reint s_99_6 -> u128
        let s_99_8: u128 = (s_99_6.value() as u128);
        // D s_99_9: size-of s_99_6
        let s_99_9: u16 = s_99_6.length();
        // D s_99_10: cast reint s_99_7 -> u128
        let s_99_10: u128 = (s_99_7.value() as u128);
        // D s_99_11: size-of s_99_7
        let s_99_11: u16 = s_99_7.length();
        // D s_99_12: lsl s_99_8 s_99_11
        let s_99_12: u128 = s_99_8 << s_99_11;
        // D s_99_13: or s_99_12 s_99_10
        let s_99_13: u128 = ((s_99_12) | (s_99_10));
        // D s_99_14: add s_99_9 s_99_11
        let s_99_14: u16 = (s_99_9 + s_99_11);
        // D s_99_15: create-bits s_99_13 s_99_14
        let s_99_15: Bits = Bits::new(s_99_13, s_99_14);
        // D s_99_16: cast reint s_99_15 -> u8
        let s_99_16: u8 = (s_99_15.value() as u8);
        // D s_99_17: cast zx s_99_16 -> bv
        let s_99_17: Bits = Bits::new(s_99_16 as u128, 2u16);
        // C s_99_18: const #3u : u8
        let s_99_18: u8 = 3;
        // C s_99_19: cast zx s_99_18 -> bv
        let s_99_19: Bits = Bits::new(s_99_18 as u128, 2u16);
        // D s_99_20: cmp-eq s_99_17 s_99_19
        let s_99_20: bool = ((s_99_17) == (s_99_19));
        // D s_99_21: write-var gs#25851 <= s_99_20
        fn_state.gs_25851 = s_99_20;
        // N s_99_22: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_100_0: read-var mshadow#488:u64
        let s_100_0: u64 = fn_state.mshadow_488;
        // D s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 64u16);
        // D s_100_2: call IsZero(s_100_1)
        let s_100_2: bool = IsZero(state, tracer, s_100_1);
        // D s_100_3: not s_100_2
        let s_100_3: bool = !s_100_2;
        // N s_100_4: branch s_100_3 b102 b101
        if s_100_3 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_101_0: const #60u : u32
        let s_101_0: u32 = 60;
        // S s_101_1: call ConstrainUnpredictableBool(s_101_0)
        let s_101_1: bool = ConstrainUnpredictableBool(state, tracer, s_101_0);
        // D s_101_2: write-var is_rejected_event <= s_101_1
        fn_state.is_rejected_event = s_101_1;
        // N s_101_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_102_0: read-var is_evt:u8
        let s_102_0: bool = fn_state.is_evt;
        // D s_102_1: not s_102_0
        let s_102_1: bool = !s_102_0;
        // D s_102_2: write-var is_rejected_event <= s_102_1
        fn_state.is_rejected_event = s_102_1;
        // N s_102_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_103_0: const #12904u : u32
        let s_103_0: u32 = 12904;
        // D s_103_1: read-reg s_103_0:u64
        let s_103_1: u64 = {
            let value = state.read_register::<u64>(s_103_0 as isize);
            tracer.read_register(s_103_0 as isize, value);
            value
        };
        // D s_103_2: cast zx s_103_1 -> bv
        let s_103_2: Bits = Bits::new(s_103_1 as u128, 64u16);
        // D s_103_3: read-var mask:u64
        let s_103_3: u64 = fn_state.mask;
        // D s_103_4: cast zx s_103_3 -> bv
        let s_103_4: Bits = Bits::new(s_103_3 as u128, 64u16);
        // D s_103_5: and s_103_2 s_103_4
        let s_103_5: Bits = ((s_103_2) & (s_103_4));
        // D s_103_6: cast reint s_103_5 -> u64
        let s_103_6: u64 = (s_103_5.value() as u64);
        // D s_103_7: write-var m <= s_103_6
        fn_state.m = s_103_6;
        // D s_103_8: read-var e:u64
        let s_103_8: u64 = fn_state.e;
        // D s_103_9: cast zx s_103_8 -> bv
        let s_103_9: Bits = Bits::new(s_103_8 as u128, 64u16);
        // D s_103_10: read-var m:u64
        let s_103_10: u64 = fn_state.m;
        // D s_103_11: cast zx s_103_10 -> bv
        let s_103_11: Bits = Bits::new(s_103_10 as u128, 64u16);
        // D s_103_12: and s_103_9 s_103_11
        let s_103_12: Bits = ((s_103_9) & (s_103_11));
        // D s_103_13: cast reint s_103_12 -> u64
        let s_103_13: u64 = (s_103_12.value() as u64);
        // D s_103_14: cast zx s_103_13 -> bv
        let s_103_14: Bits = Bits::new(s_103_13 as u128, 64u16);
        // D s_103_15: call IsZero(s_103_14)
        let s_103_15: bool = IsZero(state, tracer, s_103_14);
        // D s_103_16: write-var is_nevt <= s_103_15
        fn_state.is_nevt = s_103_15;
        // C s_103_17: const #101208u : u32
        let s_103_17: u32 = 101208;
        // D s_103_18: read-reg s_103_17:struct
        let s_103_18: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_103_17 as isize);
            tracer.read_register(s_103_17 as isize, value);
            value
        };
        // D s_103_19: call _get_PMSFCR_EL1_Type_FnE(s_103_18)
        let s_103_19: bool = u_get_PMSFCR_EL1_Type_FnE(state, tracer, s_103_18);
        // D s_103_20: cast zx s_103_19 -> bv
        let s_103_20: Bits = Bits::new(s_103_19 as u128, 1u16);
        // C s_103_21: const #1u : u8
        let s_103_21: bool = true;
        // C s_103_22: cast zx s_103_21 -> bv
        let s_103_22: Bits = Bits::new(s_103_21 as u128, 1u16);
        // D s_103_23: cmp-eq s_103_20 s_103_22
        let s_103_23: bool = ((s_103_20) == (s_103_22));
        // N s_103_24: branch s_103_23 b106 b104
        if s_103_23 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_104_0: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_105_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_106_0: read-var m:u64
        let s_106_0: u64 = fn_state.m;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 64u16);
        // D s_106_2: call IsZero(s_106_1)
        let s_106_2: bool = IsZero(state, tracer, s_106_1);
        // D s_106_3: not s_106_2
        let s_106_3: bool = !s_106_2;
        // N s_106_4: branch s_106_3 b108 b107
        if s_106_3 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_107_0: const #60u : u32
        let s_107_0: u32 = 60;
        // S s_107_1: call ConstrainUnpredictableBool(s_107_0)
        let s_107_1: bool = ConstrainUnpredictableBool(state, tracer, s_107_0);
        // D s_107_2: write-var is_rejected_nevent <= s_107_1
        fn_state.is_rejected_nevent = s_107_1;
        // N s_107_3: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_108_0: read-var is_nevt:u8
        let s_108_0: bool = fn_state.is_nevt;
        // D s_108_1: not s_108_0
        let s_108_1: bool = !s_108_0;
        // D s_108_2: write-var is_rejected_nevent <= s_108_1
        fn_state.is_rejected_nevent = s_108_1;
        // N s_108_3: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_109_0: const #31u : u8
        let s_109_0: u8 = 31;
        // C s_109_1: const #2s : i
        let s_109_1: i128 = 2;
        // C s_109_2: cast zx s_109_0 -> bv
        let s_109_2: Bits = Bits::new(s_109_0 as u128, 5u16);
        // C s_109_3: const #1s : i64
        let s_109_3: i64 = 1;
        // C s_109_4: cast zx s_109_3 -> i
        let s_109_4: i128 = (i128::try_from(s_109_3).unwrap());
        // C s_109_5: const #2s : i
        let s_109_5: i128 = 2;
        // C s_109_6: add s_109_5 s_109_4
        let s_109_6: i128 = (s_109_5 + s_109_4);
        // D s_109_7: bit-extract s_109_2 s_109_1 s_109_6
        let s_109_7: Bits = (Bits::new(
            ((s_109_2) >> (s_109_1)).value(),
            u16::try_from(s_109_6).unwrap(),
        ));
        // D s_109_8: cast reint s_109_7 -> u8
        let s_109_8: u8 = (s_109_7.value() as u8);
        // C s_109_9: const #8s : i
        let s_109_9: i128 = 8;
        // D s_109_10: read-var mask:u64
        let s_109_10: u64 = fn_state.mask;
        // D s_109_11: cast zx s_109_10 -> bv
        let s_109_11: Bits = Bits::new(s_109_10 as u128, 64u16);
        // D s_109_12: cast zx s_109_8 -> bv
        let s_109_12: Bits = Bits::new(s_109_8 as u128, 3u16);
        // C s_109_13: const #2s : i
        let s_109_13: i128 = 2;
        // C s_109_14: const #1u : u64
        let s_109_14: u64 = 1;
        // C s_109_15: cast zx s_109_14 -> bv
        let s_109_15: Bits = Bits::new(s_109_14 as u128, 64u16);
        // C s_109_16: lsl s_109_15 s_109_13
        let s_109_16: Bits = s_109_15 << s_109_13;
        // C s_109_17: sub s_109_16 s_109_15
        let s_109_17: Bits = ((s_109_16) - (s_109_15));
        // D s_109_18: and s_109_12 s_109_17
        let s_109_18: Bits = ((s_109_12) & (s_109_17));
        // D s_109_19: lsl s_109_18 s_109_9
        let s_109_19: Bits = s_109_18 << s_109_9;
        // C s_109_20: lsl s_109_17 s_109_9
        let s_109_20: Bits = s_109_17 << s_109_9;
        // C s_109_21: cmpl s_109_20
        let s_109_21: Bits = !s_109_20;
        // D s_109_22: and s_109_11 s_109_21
        let s_109_22: Bits = ((s_109_11) & (s_109_21));
        // D s_109_23: or s_109_22 s_109_19
        let s_109_23: Bits = ((s_109_22) | (s_109_19));
        // D s_109_24: cast reint s_109_23 -> u64
        let s_109_24: u64 = (s_109_23.value() as u64);
        // D s_109_25: write-var mask <= s_109_24
        fn_state.mask = s_109_24;
        // C s_109_26: const #1s : i
        let s_109_26: i128 = 1;
        // C s_109_27: cast zx s_109_0 -> bv
        let s_109_27: Bits = Bits::new(s_109_0 as u128, 5u16);
        // C s_109_28: const #1s : i64
        let s_109_28: i64 = 1;
        // C s_109_29: cast zx s_109_28 -> i
        let s_109_29: i128 = (i128::try_from(s_109_28).unwrap());
        // C s_109_30: const #0s : i
        let s_109_30: i128 = 0;
        // C s_109_31: add s_109_30 s_109_29
        let s_109_31: i128 = (s_109_30 + s_109_29);
        // D s_109_32: bit-extract s_109_27 s_109_26 s_109_31
        let s_109_32: Bits = (Bits::new(
            ((s_109_27) >> (s_109_26)).value(),
            u16::try_from(s_109_31).unwrap(),
        ));
        // D s_109_33: cast reint s_109_32 -> u8
        let s_109_33: bool = ((s_109_32.value()) != 0);
        // C s_109_34: const #4s : i
        let s_109_34: i128 = 4;
        // D s_109_35: read-var mask:u64
        let s_109_35: u64 = fn_state.mask;
        // D s_109_36: cast zx s_109_35 -> bv
        let s_109_36: Bits = Bits::new(s_109_35 as u128, 64u16);
        // D s_109_37: cast zx s_109_33 -> bv
        let s_109_37: Bits = Bits::new(s_109_33 as u128, 1u16);
        // C s_109_38: const #0s : i
        let s_109_38: i128 = 0;
        // C s_109_39: const #1u : u64
        let s_109_39: u64 = 1;
        // C s_109_40: cast zx s_109_39 -> bv
        let s_109_40: Bits = Bits::new(s_109_39 as u128, 64u16);
        // C s_109_41: lsl s_109_40 s_109_38
        let s_109_41: Bits = s_109_40 << s_109_38;
        // C s_109_42: sub s_109_41 s_109_40
        let s_109_42: Bits = ((s_109_41) - (s_109_40));
        // D s_109_43: and s_109_37 s_109_42
        let s_109_43: Bits = ((s_109_37) & (s_109_42));
        // D s_109_44: lsl s_109_43 s_109_34
        let s_109_44: Bits = s_109_43 << s_109_34;
        // C s_109_45: lsl s_109_42 s_109_34
        let s_109_45: Bits = s_109_42 << s_109_34;
        // C s_109_46: cmpl s_109_45
        let s_109_46: Bits = !s_109_45;
        // D s_109_47: and s_109_36 s_109_46
        let s_109_47: Bits = ((s_109_36) & (s_109_46));
        // D s_109_48: or s_109_47 s_109_44
        let s_109_48: Bits = ((s_109_47) | (s_109_44));
        // D s_109_49: cast reint s_109_48 -> u64
        let s_109_49: u64 = (s_109_48.value() as u64);
        // D s_109_50: write-var mask <= s_109_49
        fn_state.mask = s_109_49;
        // C s_109_51: const #0s : i
        let s_109_51: i128 = 0;
        // C s_109_52: cast zx s_109_0 -> bv
        let s_109_52: Bits = Bits::new(s_109_0 as u128, 5u16);
        // C s_109_53: const #1s : i64
        let s_109_53: i64 = 1;
        // C s_109_54: cast zx s_109_53 -> i
        let s_109_54: i128 = (i128::try_from(s_109_53).unwrap());
        // C s_109_55: const #0s : i
        let s_109_55: i128 = 0;
        // C s_109_56: add s_109_55 s_109_54
        let s_109_56: i128 = (s_109_55 + s_109_54);
        // D s_109_57: bit-extract s_109_52 s_109_51 s_109_56
        let s_109_57: Bits = (Bits::new(
            ((s_109_52) >> (s_109_51)).value(),
            u16::try_from(s_109_56).unwrap(),
        ));
        // D s_109_58: cast reint s_109_57 -> u8
        let s_109_58: bool = ((s_109_57.value()) != 0);
        // C s_109_59: const #2s : i
        let s_109_59: i128 = 2;
        // D s_109_60: read-var mask:u64
        let s_109_60: u64 = fn_state.mask;
        // D s_109_61: cast zx s_109_60 -> bv
        let s_109_61: Bits = Bits::new(s_109_60 as u128, 64u16);
        // D s_109_62: cast zx s_109_58 -> bv
        let s_109_62: Bits = Bits::new(s_109_58 as u128, 1u16);
        // C s_109_63: const #0s : i
        let s_109_63: i128 = 0;
        // C s_109_64: const #1u : u64
        let s_109_64: u64 = 1;
        // C s_109_65: cast zx s_109_64 -> bv
        let s_109_65: Bits = Bits::new(s_109_64 as u128, 64u16);
        // C s_109_66: lsl s_109_65 s_109_63
        let s_109_66: Bits = s_109_65 << s_109_63;
        // C s_109_67: sub s_109_66 s_109_65
        let s_109_67: Bits = ((s_109_66) - (s_109_65));
        // D s_109_68: and s_109_62 s_109_67
        let s_109_68: Bits = ((s_109_62) & (s_109_67));
        // D s_109_69: lsl s_109_68 s_109_59
        let s_109_69: Bits = s_109_68 << s_109_59;
        // C s_109_70: lsl s_109_67 s_109_59
        let s_109_70: Bits = s_109_67 << s_109_59;
        // C s_109_71: cmpl s_109_70
        let s_109_71: Bits = !s_109_70;
        // D s_109_72: and s_109_61 s_109_71
        let s_109_72: Bits = ((s_109_61) & (s_109_71));
        // D s_109_73: or s_109_72 s_109_69
        let s_109_73: Bits = ((s_109_72) | (s_109_69));
        // D s_109_74: cast reint s_109_73 -> u64
        let s_109_74: u64 = (s_109_73.value() as u64);
        // D s_109_75: write-var mask <= s_109_74
        fn_state.mask = s_109_74;
        // N s_109_76: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_110_0: const #1u : u8
        let s_110_0: bool = true;
        // S s_110_1: call Bit(s_110_0)
        let s_110_1: bool = Bit(state, tracer, s_110_0);
        // C s_110_2: const #6s : i
        let s_110_2: i128 = 6;
        // D s_110_3: read-var mask:u64
        let s_110_3: u64 = fn_state.mask;
        // D s_110_4: cast zx s_110_3 -> bv
        let s_110_4: Bits = Bits::new(s_110_3 as u128, 64u16);
        // C s_110_5: const #1u : u64
        let s_110_5: u64 = 1;
        // D s_110_6: bit-insert s_110_4 s_110_4 s_110_2 s_110_5
        let s_110_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_110_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_110_4.length(),
            );
            (s_110_4 & mask) | (s_110_4 << s_110_2)
        };
        // D s_110_7: cast reint s_110_6 -> u64
        let s_110_7: u64 = (s_110_6.value() as u64);
        // D s_110_8: write-var mask <= s_110_7
        fn_state.mask = s_110_7;
        // N s_110_9: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_111_0: const #1u : u8
        let s_111_0: bool = true;
        // S s_111_1: call Bit(s_111_0)
        let s_111_1: bool = Bit(state, tracer, s_111_0);
        // C s_111_2: const #11s : i
        let s_111_2: i128 = 11;
        // D s_111_3: read-var mask:u64
        let s_111_3: u64 = fn_state.mask;
        // D s_111_4: cast zx s_111_3 -> bv
        let s_111_4: Bits = Bits::new(s_111_3 as u128, 64u16);
        // C s_111_5: const #1u : u64
        let s_111_5: u64 = 1;
        // D s_111_6: bit-insert s_111_4 s_111_4 s_111_2 s_111_5
        let s_111_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_111_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_111_4.length(),
            );
            (s_111_4 & mask) | (s_111_4 << s_111_2)
        };
        // D s_111_7: cast reint s_111_6 -> u64
        let s_111_7: u64 = (s_111_6.value() as u64);
        // D s_111_8: write-var mask <= s_111_7
        fn_state.mask = s_111_7;
        // N s_111_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_112_0: const #1u : u8
        let s_112_0: bool = true;
        // S s_112_1: call Bit(s_112_0)
        let s_112_1: bool = Bit(state, tracer, s_112_0);
        // C s_112_2: const #16s : i
        let s_112_2: i128 = 16;
        // D s_112_3: read-var mask:u64
        let s_112_3: u64 = fn_state.mask;
        // D s_112_4: cast zx s_112_3 -> bv
        let s_112_4: Bits = Bits::new(s_112_3 as u128, 64u16);
        // C s_112_5: const #1u : u64
        let s_112_5: u64 = 1;
        // D s_112_6: bit-insert s_112_4 s_112_4 s_112_2 s_112_5
        let s_112_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_112_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_112_4.length(),
            );
            (s_112_4 & mask) | (s_112_4 << s_112_2)
        };
        // D s_112_7: cast reint s_112_6 -> u64
        let s_112_7: u64 = (s_112_6.value() as u64);
        // D s_112_8: write-var mask <= s_112_7
        fn_state.mask = s_112_7;
        // N s_112_9: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_113_0: const #17s : i
        let s_113_0: i128 = 17;
        // D s_113_1: read-var mask:u64
        let s_113_1: u64 = fn_state.mask;
        // D s_113_2: cast zx s_113_1 -> bv
        let s_113_2: Bits = Bits::new(s_113_1 as u128, 64u16);
        // C s_113_3: const #3u : u8
        let s_113_3: u8 = 3;
        // C s_113_4: cast zx s_113_3 -> bv
        let s_113_4: Bits = Bits::new(s_113_3 as u128, 2u16);
        // C s_113_5: const #1s : i
        let s_113_5: i128 = 1;
        // C s_113_6: const #1u : u64
        let s_113_6: u64 = 1;
        // C s_113_7: cast zx s_113_6 -> bv
        let s_113_7: Bits = Bits::new(s_113_6 as u128, 64u16);
        // C s_113_8: lsl s_113_7 s_113_5
        let s_113_8: Bits = s_113_7 << s_113_5;
        // C s_113_9: sub s_113_8 s_113_7
        let s_113_9: Bits = ((s_113_8) - (s_113_7));
        // C s_113_10: and s_113_4 s_113_9
        let s_113_10: Bits = ((s_113_4) & (s_113_9));
        // C s_113_11: lsl s_113_10 s_113_0
        let s_113_11: Bits = s_113_10 << s_113_0;
        // C s_113_12: lsl s_113_9 s_113_0
        let s_113_12: Bits = s_113_9 << s_113_0;
        // C s_113_13: cmpl s_113_12
        let s_113_13: Bits = !s_113_12;
        // D s_113_14: and s_113_2 s_113_13
        let s_113_14: Bits = ((s_113_2) & (s_113_13));
        // D s_113_15: or s_113_14 s_113_11
        let s_113_15: Bits = ((s_113_14) | (s_113_11));
        // D s_113_16: cast reint s_113_15 -> u64
        let s_113_16: u64 = (s_113_15.value() as u64);
        // D s_113_17: write-var mask <= s_113_16
        fn_state.mask = s_113_16;
        // N s_113_18: jump b2
        return block_2(state, tracer, fn_state);
    }
}
