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
use SetPSTATEFromPSR::*;
use u__UNKNOWN_bits::*;
use DLR_write::*;
use DSPSR_write::*;
use Mk_DSPSR_Type::*;
use UsingAArch32::*;
use UpdateEDSCRFields::*;
use Mk_DSPSR_EL0_Type::*;
use SPSR_read::*;
use common::*;
pub fn DebugRestorePSR<T: Tracer>(state: &mut State, tracer: &T, gs_29444: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_29444: (),
    }
    let fn_state = FunctionState {
        gs_29444,
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
        // S s_0_1: call UsingAArch32(s_0_0)
        let s_0_1: bool = UsingAArch32(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b3 b1
        if s_0_1 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i
        let s_1_0: i128 = 64;
        // S s_1_1: call SPSR_read(s_1_0)
        let s_1_1: Bits = SPSR_read(state, tracer, s_1_0);
        // S s_1_2: cast reint s_1_1 -> u64
        let s_1_2: u64 = (s_1_1.value() as u64);
        // S s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 64u16);
        // S s_1_4: call SetPSTATEFromPSR(s_1_3)
        let s_1_4: () = SetPSTATEFromPSR(state, tracer, s_1_3);
        // C s_1_5: const #9s : i64
        let s_1_5: i64 = 9;
        // C s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // S s_1_7: call __UNKNOWN_bits(s_1_6)
        let s_1_7: Bits = u__UNKNOWN_bits(state, tracer, s_1_6);
        // S s_1_8: cast reint s_1_7 -> u9
        let s_1_8: u16 = (s_1_7.value() as u16);
        // C s_1_9: const #8s : i
        let s_1_9: i128 = 8;
        // S s_1_10: cast zx s_1_8 -> bv
        let s_1_10: Bits = Bits::new(s_1_8 as u128, 9u16);
        // C s_1_11: const #1s : i64
        let s_1_11: i64 = 1;
        // C s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // C s_1_13: const #0s : i
        let s_1_13: i128 = 0;
        // C s_1_14: add s_1_13 s_1_12
        let s_1_14: i128 = (s_1_13 + s_1_12);
        // D s_1_15: bit-extract s_1_10 s_1_9 s_1_14
        let s_1_15: Bits = (Bits::new(
            ((s_1_10) >> (s_1_9)).value(),
            u16::try_from(s_1_14).unwrap(),
        ));
        // D s_1_16: cast reint s_1_15 -> u8
        let s_1_16: bool = ((s_1_15.value()) != 0);
        // C s_1_17: const #16984u : u32
        let s_1_17: u32 = 16984;
        // N s_1_18: write-reg s_1_17 <= s_1_16
        let s_1_18: () = {
            state.write_register::<bool>(s_1_17 as isize, s_1_16);
            tracer.write_register(s_1_17 as isize, s_1_16);
        };
        // C s_1_19: const #7s : i
        let s_1_19: i128 = 7;
        // S s_1_20: cast zx s_1_8 -> bv
        let s_1_20: Bits = Bits::new(s_1_8 as u128, 9u16);
        // C s_1_21: const #1s : i64
        let s_1_21: i64 = 1;
        // C s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // C s_1_23: const #0s : i
        let s_1_23: i128 = 0;
        // C s_1_24: add s_1_23 s_1_22
        let s_1_24: i128 = (s_1_23 + s_1_22);
        // D s_1_25: bit-extract s_1_20 s_1_19 s_1_24
        let s_1_25: Bits = (Bits::new(
            ((s_1_20) >> (s_1_19)).value(),
            u16::try_from(s_1_24).unwrap(),
        ));
        // D s_1_26: cast reint s_1_25 -> u8
        let s_1_26: bool = ((s_1_25.value()) != 0);
        // C s_1_27: const #16997u : u32
        let s_1_27: u32 = 16997;
        // N s_1_28: write-reg s_1_27 <= s_1_26
        let s_1_28: () = {
            state.write_register::<bool>(s_1_27 as isize, s_1_26);
            tracer.write_register(s_1_27 as isize, s_1_26);
        };
        // C s_1_29: const #6s : i
        let s_1_29: i128 = 6;
        // S s_1_30: cast zx s_1_8 -> bv
        let s_1_30: Bits = Bits::new(s_1_8 as u128, 9u16);
        // C s_1_31: const #1s : i64
        let s_1_31: i64 = 1;
        // C s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // C s_1_33: const #0s : i
        let s_1_33: i128 = 0;
        // C s_1_34: add s_1_33 s_1_32
        let s_1_34: i128 = (s_1_33 + s_1_32);
        // D s_1_35: bit-extract s_1_30 s_1_29 s_1_34
        let s_1_35: Bits = (Bits::new(
            ((s_1_30) >> (s_1_29)).value(),
            u16::try_from(s_1_34).unwrap(),
        ));
        // D s_1_36: cast reint s_1_35 -> u8
        let s_1_36: bool = ((s_1_35.value()) != 0);
        // C s_1_37: const #16971u : u32
        let s_1_37: u32 = 16971;
        // N s_1_38: write-reg s_1_37 <= s_1_36
        let s_1_38: () = {
            state.write_register::<bool>(s_1_37 as isize, s_1_36);
            tracer.write_register(s_1_37 as isize, s_1_36);
        };
        // C s_1_39: const #5s : i
        let s_1_39: i128 = 5;
        // S s_1_40: cast zx s_1_8 -> bv
        let s_1_40: Bits = Bits::new(s_1_8 as u128, 9u16);
        // C s_1_41: const #1s : i64
        let s_1_41: i64 = 1;
        // C s_1_42: cast zx s_1_41 -> i
        let s_1_42: i128 = (i128::try_from(s_1_41).unwrap());
        // C s_1_43: const #0s : i
        let s_1_43: i128 = 0;
        // C s_1_44: add s_1_43 s_1_42
        let s_1_44: i128 = (s_1_43 + s_1_42);
        // D s_1_45: bit-extract s_1_40 s_1_39 s_1_44
        let s_1_45: Bits = (Bits::new(
            ((s_1_40) >> (s_1_39)).value(),
            u16::try_from(s_1_44).unwrap(),
        ));
        // D s_1_46: cast reint s_1_45 -> u8
        let s_1_46: bool = ((s_1_45.value()) != 0);
        // C s_1_47: const #16996u : u32
        let s_1_47: u32 = 16996;
        // N s_1_48: write-reg s_1_47 <= s_1_46
        let s_1_48: () = {
            state.write_register::<bool>(s_1_47 as isize, s_1_46);
            tracer.write_register(s_1_47 as isize, s_1_46);
        };
        // C s_1_49: const #4s : i
        let s_1_49: i128 = 4;
        // S s_1_50: cast zx s_1_8 -> bv
        let s_1_50: Bits = Bits::new(s_1_8 as u128, 9u16);
        // C s_1_51: const #1s : i64
        let s_1_51: i64 = 1;
        // C s_1_52: cast zx s_1_51 -> i
        let s_1_52: i128 = (i128::try_from(s_1_51).unwrap());
        // C s_1_53: const #0s : i
        let s_1_53: i128 = 0;
        // C s_1_54: add s_1_53 s_1_52
        let s_1_54: i128 = (s_1_53 + s_1_52);
        // D s_1_55: bit-extract s_1_50 s_1_49 s_1_54
        let s_1_55: Bits = (Bits::new(
            ((s_1_50) >> (s_1_49)).value(),
            u16::try_from(s_1_54).unwrap(),
        ));
        // D s_1_56: cast reint s_1_55 -> u8
        let s_1_56: bool = ((s_1_55.value()) != 0);
        // C s_1_57: const #16991u : u32
        let s_1_57: u32 = 16991;
        // N s_1_58: write-reg s_1_57 <= s_1_56
        let s_1_58: () = {
            state.write_register::<bool>(s_1_57 as isize, s_1_56);
            tracer.write_register(s_1_57 as isize, s_1_56);
        };
        // C s_1_59: const #3s : i
        let s_1_59: i128 = 3;
        // S s_1_60: cast zx s_1_8 -> bv
        let s_1_60: Bits = Bits::new(s_1_8 as u128, 9u16);
        // C s_1_61: const #1s : i64
        let s_1_61: i64 = 1;
        // C s_1_62: cast zx s_1_61 -> i
        let s_1_62: i128 = (i128::try_from(s_1_61).unwrap());
        // C s_1_63: const #0s : i
        let s_1_63: i128 = 0;
        // C s_1_64: add s_1_63 s_1_62
        let s_1_64: i128 = (s_1_63 + s_1_62);
        // D s_1_65: bit-extract s_1_60 s_1_59 s_1_64
        let s_1_65: Bits = (Bits::new(
            ((s_1_60) >> (s_1_59)).value(),
            u16::try_from(s_1_64).unwrap(),
        ));
        // D s_1_66: cast reint s_1_65 -> u8
        let s_1_66: bool = ((s_1_65.value()) != 0);
        // C s_1_67: const #16972u : u32
        let s_1_67: u32 = 16972;
        // N s_1_68: write-reg s_1_67 <= s_1_66
        let s_1_68: () = {
            state.write_register::<bool>(s_1_67 as isize, s_1_66);
            tracer.write_register(s_1_67 as isize, s_1_66);
        };
        // C s_1_69: const #2s : i
        let s_1_69: i128 = 2;
        // S s_1_70: cast zx s_1_8 -> bv
        let s_1_70: Bits = Bits::new(s_1_8 as u128, 9u16);
        // C s_1_71: const #1s : i64
        let s_1_71: i64 = 1;
        // C s_1_72: cast zx s_1_71 -> i
        let s_1_72: i128 = (i128::try_from(s_1_71).unwrap());
        // C s_1_73: const #0s : i
        let s_1_73: i128 = 0;
        // C s_1_74: add s_1_73 s_1_72
        let s_1_74: i128 = (s_1_73 + s_1_72);
        // D s_1_75: bit-extract s_1_70 s_1_69 s_1_74
        let s_1_75: Bits = (Bits::new(
            ((s_1_70) >> (s_1_69)).value(),
            u16::try_from(s_1_74).unwrap(),
        ));
        // D s_1_76: cast reint s_1_75 -> u8
        let s_1_76: bool = ((s_1_75.value()) != 0);
        // C s_1_77: const #16968u : u32
        let s_1_77: u32 = 16968;
        // N s_1_78: write-reg s_1_77 <= s_1_76
        let s_1_78: () = {
            state.write_register::<bool>(s_1_77 as isize, s_1_76);
            tracer.write_register(s_1_77 as isize, s_1_76);
        };
        // C s_1_79: const #1s : i
        let s_1_79: i128 = 1;
        // S s_1_80: cast zx s_1_8 -> bv
        let s_1_80: Bits = Bits::new(s_1_8 as u128, 9u16);
        // C s_1_81: const #1s : i64
        let s_1_81: i64 = 1;
        // C s_1_82: cast zx s_1_81 -> i
        let s_1_82: i128 = (i128::try_from(s_1_81).unwrap());
        // C s_1_83: const #0s : i
        let s_1_83: i128 = 0;
        // C s_1_84: add s_1_83 s_1_82
        let s_1_84: i128 = (s_1_83 + s_1_82);
        // D s_1_85: bit-extract s_1_80 s_1_79 s_1_84
        let s_1_85: Bits = (Bits::new(
            ((s_1_80) >> (s_1_79)).value(),
            u16::try_from(s_1_84).unwrap(),
        ));
        // D s_1_86: cast reint s_1_85 -> u8
        let s_1_86: bool = ((s_1_85.value()) != 0);
        // C s_1_87: const #16979u : u32
        let s_1_87: u32 = 16979;
        // N s_1_88: write-reg s_1_87 <= s_1_86
        let s_1_88: () = {
            state.write_register::<bool>(s_1_87 as isize, s_1_86);
            tracer.write_register(s_1_87 as isize, s_1_86);
        };
        // C s_1_89: const #0s : i
        let s_1_89: i128 = 0;
        // S s_1_90: cast zx s_1_8 -> bv
        let s_1_90: Bits = Bits::new(s_1_8 as u128, 9u16);
        // C s_1_91: const #1s : i64
        let s_1_91: i64 = 1;
        // C s_1_92: cast zx s_1_91 -> i
        let s_1_92: i128 = (i128::try_from(s_1_91).unwrap());
        // C s_1_93: const #0s : i
        let s_1_93: i128 = 0;
        // C s_1_94: add s_1_93 s_1_92
        let s_1_94: i128 = (s_1_93 + s_1_92);
        // D s_1_95: bit-extract s_1_90 s_1_89 s_1_94
        let s_1_95: Bits = (Bits::new(
            ((s_1_90) >> (s_1_89)).value(),
            u16::try_from(s_1_94).unwrap(),
        ));
        // D s_1_96: cast reint s_1_95 -> u8
        let s_1_96: bool = ((s_1_95.value()) != 0);
        // C s_1_97: const #16977u : u32
        let s_1_97: u32 = 16977;
        // N s_1_98: write-reg s_1_97 <= s_1_96
        let s_1_98: () = {
            state.write_register::<bool>(s_1_97 as isize, s_1_96);
            tracer.write_register(s_1_97 as isize, s_1_96);
        };
        // C s_1_99: const #64s : i64
        let s_1_99: i64 = 64;
        // C s_1_100: cast zx s_1_99 -> i
        let s_1_100: i128 = (i128::try_from(s_1_99).unwrap());
        // S s_1_101: call __UNKNOWN_bits(s_1_100)
        let s_1_101: Bits = u__UNKNOWN_bits(state, tracer, s_1_100);
        // S s_1_102: cast reint s_1_101 -> u64
        let s_1_102: u64 = (s_1_101.value() as u64);
        // C s_1_103: const #13360u : u32
        let s_1_103: u32 = 13360;
        // N s_1_104: write-reg s_1_103 <= s_1_102
        let s_1_104: () = {
            state.write_register::<u64>(s_1_103 as isize, s_1_102);
            tracer.write_register(s_1_103 as isize, s_1_102);
        };
        // C s_1_105: const #64s : i64
        let s_1_105: i64 = 64;
        // C s_1_106: cast zx s_1_105 -> i
        let s_1_106: i128 = (i128::try_from(s_1_105).unwrap());
        // S s_1_107: call __UNKNOWN_bits(s_1_106)
        let s_1_107: Bits = u__UNKNOWN_bits(state, tracer, s_1_106);
        // S s_1_108: cast reint s_1_107 -> u64
        let s_1_108: u64 = (s_1_107.value() as u64);
        // S s_1_109: call Mk_DSPSR_EL0_Type(s_1_108)
        let s_1_109: ProductType5c790c8ef59cc8b2 = Mk_DSPSR_EL0_Type(
            state,
            tracer,
            s_1_108,
        );
        // C s_1_110: const #102584u : u32
        let s_1_110: u32 = 102584;
        // N s_1_111: write-reg s_1_110 <= s_1_109
        let s_1_111: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_1_110 as isize, s_1_109);
            tracer.write_register(s_1_110 as isize, s_1_109);
        };
        // N s_1_112: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call UpdateEDSCRFields(s_2_0)
        let s_2_1: () = UpdateEDSCRFields(state, tracer, s_2_0);
        // N s_2_2: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #32s : i
        let s_3_0: i128 = 32;
        // S s_3_1: call SPSR_read(s_3_0)
        let s_3_1: Bits = SPSR_read(state, tracer, s_3_0);
        // S s_3_2: cast reint s_3_1 -> u32
        let s_3_2: u32 = (s_3_1.value() as u32);
        // S s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 32u16);
        // S s_3_4: call SetPSTATEFromPSR(s_3_3)
        let s_3_4: () = SetPSTATEFromPSR(state, tracer, s_3_3);
        // C s_3_5: const #13s : i64
        let s_3_5: i64 = 13;
        // C s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // S s_3_7: call __UNKNOWN_bits(s_3_6)
        let s_3_7: Bits = u__UNKNOWN_bits(state, tracer, s_3_6);
        // S s_3_8: cast reint s_3_7 -> u13
        let s_3_8: u16 = (s_3_7.value() as u16);
        // C s_3_9: const #12s : i
        let s_3_9: i128 = 12;
        // S s_3_10: cast zx s_3_8 -> bv
        let s_3_10: Bits = Bits::new(s_3_8 as u128, 13u16);
        // C s_3_11: const #1s : i64
        let s_3_11: i64 = 1;
        // C s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // C s_3_13: const #0s : i
        let s_3_13: i128 = 0;
        // C s_3_14: add s_3_13 s_3_12
        let s_3_14: i128 = (s_3_13 + s_3_12);
        // D s_3_15: bit-extract s_3_10 s_3_9 s_3_14
        let s_3_15: Bits = (Bits::new(
            ((s_3_10) >> (s_3_9)).value(),
            u16::try_from(s_3_14).unwrap(),
        ));
        // D s_3_16: cast reint s_3_15 -> u8
        let s_3_16: bool = ((s_3_15.value()) != 0);
        // C s_3_17: const #16984u : u32
        let s_3_17: u32 = 16984;
        // N s_3_18: write-reg s_3_17 <= s_3_16
        let s_3_18: () = {
            state.write_register::<bool>(s_3_17 as isize, s_3_16);
            tracer.write_register(s_3_17 as isize, s_3_16);
        };
        // C s_3_19: const #11s : i
        let s_3_19: i128 = 11;
        // S s_3_20: cast zx s_3_8 -> bv
        let s_3_20: Bits = Bits::new(s_3_8 as u128, 13u16);
        // C s_3_21: const #1s : i64
        let s_3_21: i64 = 1;
        // C s_3_22: cast zx s_3_21 -> i
        let s_3_22: i128 = (i128::try_from(s_3_21).unwrap());
        // C s_3_23: const #0s : i
        let s_3_23: i128 = 0;
        // C s_3_24: add s_3_23 s_3_22
        let s_3_24: i128 = (s_3_23 + s_3_22);
        // D s_3_25: bit-extract s_3_20 s_3_19 s_3_24
        let s_3_25: Bits = (Bits::new(
            ((s_3_20) >> (s_3_19)).value(),
            u16::try_from(s_3_24).unwrap(),
        ));
        // D s_3_26: cast reint s_3_25 -> u8
        let s_3_26: bool = ((s_3_25.value()) != 0);
        // C s_3_27: const #16997u : u32
        let s_3_27: u32 = 16997;
        // N s_3_28: write-reg s_3_27 <= s_3_26
        let s_3_28: () = {
            state.write_register::<bool>(s_3_27 as isize, s_3_26);
            tracer.write_register(s_3_27 as isize, s_3_26);
        };
        // C s_3_29: const #10s : i
        let s_3_29: i128 = 10;
        // S s_3_30: cast zx s_3_8 -> bv
        let s_3_30: Bits = Bits::new(s_3_8 as u128, 13u16);
        // C s_3_31: const #1s : i64
        let s_3_31: i64 = 1;
        // C s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // C s_3_33: const #0s : i
        let s_3_33: i128 = 0;
        // C s_3_34: add s_3_33 s_3_32
        let s_3_34: i128 = (s_3_33 + s_3_32);
        // D s_3_35: bit-extract s_3_30 s_3_29 s_3_34
        let s_3_35: Bits = (Bits::new(
            ((s_3_30) >> (s_3_29)).value(),
            u16::try_from(s_3_34).unwrap(),
        ));
        // D s_3_36: cast reint s_3_35 -> u8
        let s_3_36: bool = ((s_3_35.value()) != 0);
        // C s_3_37: const #16971u : u32
        let s_3_37: u32 = 16971;
        // N s_3_38: write-reg s_3_37 <= s_3_36
        let s_3_38: () = {
            state.write_register::<bool>(s_3_37 as isize, s_3_36);
            tracer.write_register(s_3_37 as isize, s_3_36);
        };
        // C s_3_39: const #9s : i
        let s_3_39: i128 = 9;
        // S s_3_40: cast zx s_3_8 -> bv
        let s_3_40: Bits = Bits::new(s_3_8 as u128, 13u16);
        // C s_3_41: const #1s : i64
        let s_3_41: i64 = 1;
        // C s_3_42: cast zx s_3_41 -> i
        let s_3_42: i128 = (i128::try_from(s_3_41).unwrap());
        // C s_3_43: const #0s : i
        let s_3_43: i128 = 0;
        // C s_3_44: add s_3_43 s_3_42
        let s_3_44: i128 = (s_3_43 + s_3_42);
        // D s_3_45: bit-extract s_3_40 s_3_39 s_3_44
        let s_3_45: Bits = (Bits::new(
            ((s_3_40) >> (s_3_39)).value(),
            u16::try_from(s_3_44).unwrap(),
        ));
        // D s_3_46: cast reint s_3_45 -> u8
        let s_3_46: bool = ((s_3_45.value()) != 0);
        // C s_3_47: const #16996u : u32
        let s_3_47: u32 = 16996;
        // N s_3_48: write-reg s_3_47 <= s_3_46
        let s_3_48: () = {
            state.write_register::<bool>(s_3_47 as isize, s_3_46);
            tracer.write_register(s_3_47 as isize, s_3_46);
        };
        // C s_3_49: const #8s : i
        let s_3_49: i128 = 8;
        // S s_3_50: cast zx s_3_8 -> bv
        let s_3_50: Bits = Bits::new(s_3_8 as u128, 13u16);
        // C s_3_51: const #1s : i64
        let s_3_51: i64 = 1;
        // C s_3_52: cast zx s_3_51 -> i
        let s_3_52: i128 = (i128::try_from(s_3_51).unwrap());
        // C s_3_53: const #0s : i
        let s_3_53: i128 = 0;
        // C s_3_54: add s_3_53 s_3_52
        let s_3_54: i128 = (s_3_53 + s_3_52);
        // D s_3_55: bit-extract s_3_50 s_3_49 s_3_54
        let s_3_55: Bits = (Bits::new(
            ((s_3_50) >> (s_3_49)).value(),
            u16::try_from(s_3_54).unwrap(),
        ));
        // D s_3_56: cast reint s_3_55 -> u8
        let s_3_56: bool = ((s_3_55.value()) != 0);
        // C s_3_57: const #16988u : u32
        let s_3_57: u32 = 16988;
        // N s_3_58: write-reg s_3_57 <= s_3_56
        let s_3_58: () = {
            state.write_register::<bool>(s_3_57 as isize, s_3_56);
            tracer.write_register(s_3_57 as isize, s_3_56);
        };
        // C s_3_59: const #4s : i
        let s_3_59: i128 = 4;
        // S s_3_60: cast zx s_3_8 -> bv
        let s_3_60: Bits = Bits::new(s_3_8 as u128, 13u16);
        // C s_3_61: const #1s : i64
        let s_3_61: i64 = 1;
        // C s_3_62: cast zx s_3_61 -> i
        let s_3_62: i128 = (i128::try_from(s_3_61).unwrap());
        // C s_3_63: const #3s : i
        let s_3_63: i128 = 3;
        // C s_3_64: add s_3_63 s_3_62
        let s_3_64: i128 = (s_3_63 + s_3_62);
        // D s_3_65: bit-extract s_3_60 s_3_59 s_3_64
        let s_3_65: Bits = (Bits::new(
            ((s_3_60) >> (s_3_59)).value(),
            u16::try_from(s_3_64).unwrap(),
        ));
        // D s_3_66: cast reint s_3_65 -> u8
        let s_3_66: u8 = (s_3_65.value() as u8);
        // C s_3_67: const #16978u : u32
        let s_3_67: u32 = 16978;
        // N s_3_68: write-reg s_3_67 <= s_3_66
        let s_3_68: () = {
            state.write_register::<u8>(s_3_67 as isize, s_3_66);
            tracer.write_register(s_3_67 as isize, s_3_66);
        };
        // C s_3_69: const #3s : i
        let s_3_69: i128 = 3;
        // S s_3_70: cast zx s_3_8 -> bv
        let s_3_70: Bits = Bits::new(s_3_8 as u128, 13u16);
        // C s_3_71: const #1s : i64
        let s_3_71: i64 = 1;
        // C s_3_72: cast zx s_3_71 -> i
        let s_3_72: i128 = (i128::try_from(s_3_71).unwrap());
        // C s_3_73: const #0s : i
        let s_3_73: i128 = 0;
        // C s_3_74: add s_3_73 s_3_72
        let s_3_74: i128 = (s_3_73 + s_3_72);
        // D s_3_75: bit-extract s_3_70 s_3_69 s_3_74
        let s_3_75: Bits = (Bits::new(
            ((s_3_70) >> (s_3_69)).value(),
            u16::try_from(s_3_74).unwrap(),
        ));
        // D s_3_76: cast reint s_3_75 -> u8
        let s_3_76: bool = ((s_3_75.value()) != 0);
        // C s_3_77: const #16991u : u32
        let s_3_77: u32 = 16991;
        // N s_3_78: write-reg s_3_77 <= s_3_76
        let s_3_78: () = {
            state.write_register::<bool>(s_3_77 as isize, s_3_76);
            tracer.write_register(s_3_77 as isize, s_3_76);
        };
        // C s_3_79: const #2s : i
        let s_3_79: i128 = 2;
        // S s_3_80: cast zx s_3_8 -> bv
        let s_3_80: Bits = Bits::new(s_3_8 as u128, 13u16);
        // C s_3_81: const #1s : i64
        let s_3_81: i64 = 1;
        // C s_3_82: cast zx s_3_81 -> i
        let s_3_82: i128 = (i128::try_from(s_3_81).unwrap());
        // C s_3_83: const #0s : i
        let s_3_83: i128 = 0;
        // C s_3_84: add s_3_83 s_3_82
        let s_3_84: i128 = (s_3_83 + s_3_82);
        // D s_3_85: bit-extract s_3_80 s_3_79 s_3_84
        let s_3_85: Bits = (Bits::new(
            ((s_3_80) >> (s_3_79)).value(),
            u16::try_from(s_3_84).unwrap(),
        ));
        // D s_3_86: cast reint s_3_85 -> u8
        let s_3_86: bool = ((s_3_85.value()) != 0);
        // C s_3_87: const #16968u : u32
        let s_3_87: u32 = 16968;
        // N s_3_88: write-reg s_3_87 <= s_3_86
        let s_3_88: () = {
            state.write_register::<bool>(s_3_87 as isize, s_3_86);
            tracer.write_register(s_3_87 as isize, s_3_86);
        };
        // C s_3_89: const #1s : i
        let s_3_89: i128 = 1;
        // S s_3_90: cast zx s_3_8 -> bv
        let s_3_90: Bits = Bits::new(s_3_8 as u128, 13u16);
        // C s_3_91: const #1s : i64
        let s_3_91: i64 = 1;
        // C s_3_92: cast zx s_3_91 -> i
        let s_3_92: i128 = (i128::try_from(s_3_91).unwrap());
        // C s_3_93: const #0s : i
        let s_3_93: i128 = 0;
        // C s_3_94: add s_3_93 s_3_92
        let s_3_94: i128 = (s_3_93 + s_3_92);
        // D s_3_95: bit-extract s_3_90 s_3_89 s_3_94
        let s_3_95: Bits = (Bits::new(
            ((s_3_90) >> (s_3_89)).value(),
            u16::try_from(s_3_94).unwrap(),
        ));
        // D s_3_96: cast reint s_3_95 -> u8
        let s_3_96: bool = ((s_3_95.value()) != 0);
        // C s_3_97: const #16979u : u32
        let s_3_97: u32 = 16979;
        // N s_3_98: write-reg s_3_97 <= s_3_96
        let s_3_98: () = {
            state.write_register::<bool>(s_3_97 as isize, s_3_96);
            tracer.write_register(s_3_97 as isize, s_3_96);
        };
        // C s_3_99: const #0s : i
        let s_3_99: i128 = 0;
        // S s_3_100: cast zx s_3_8 -> bv
        let s_3_100: Bits = Bits::new(s_3_8 as u128, 13u16);
        // C s_3_101: const #1s : i64
        let s_3_101: i64 = 1;
        // C s_3_102: cast zx s_3_101 -> i
        let s_3_102: i128 = (i128::try_from(s_3_101).unwrap());
        // C s_3_103: const #0s : i
        let s_3_103: i128 = 0;
        // C s_3_104: add s_3_103 s_3_102
        let s_3_104: i128 = (s_3_103 + s_3_102);
        // D s_3_105: bit-extract s_3_100 s_3_99 s_3_104
        let s_3_105: Bits = (Bits::new(
            ((s_3_100) >> (s_3_99)).value(),
            u16::try_from(s_3_104).unwrap(),
        ));
        // D s_3_106: cast reint s_3_105 -> u8
        let s_3_106: bool = ((s_3_105.value()) != 0);
        // C s_3_107: const #16977u : u32
        let s_3_107: u32 = 16977;
        // N s_3_108: write-reg s_3_107 <= s_3_106
        let s_3_108: () = {
            state.write_register::<bool>(s_3_107 as isize, s_3_106);
            tracer.write_register(s_3_107 as isize, s_3_106);
        };
        // C s_3_109: const #0u : u8
        let s_3_109: u8 = 0;
        // C s_3_110: const #16981u : u32
        let s_3_110: u32 = 16981;
        // N s_3_111: write-reg s_3_110 <= s_3_109
        let s_3_111: () = {
            state.write_register::<u8>(s_3_110 as isize, s_3_109);
            tracer.write_register(s_3_110 as isize, s_3_109);
        };
        // C s_3_112: const #1u : u8
        let s_3_112: bool = true;
        // C s_3_113: const #16993u : u32
        let s_3_113: u32 = 16993;
        // N s_3_114: write-reg s_3_113 <= s_3_112
        let s_3_114: () = {
            state.write_register::<bool>(s_3_113 as isize, s_3_112);
            tracer.write_register(s_3_113 as isize, s_3_112);
        };
        // C s_3_115: const #32s : i64
        let s_3_115: i64 = 32;
        // C s_3_116: cast zx s_3_115 -> i
        let s_3_116: i128 = (i128::try_from(s_3_115).unwrap());
        // S s_3_117: call __UNKNOWN_bits(s_3_116)
        let s_3_117: Bits = u__UNKNOWN_bits(state, tracer, s_3_116);
        // S s_3_118: cast reint s_3_117 -> u32
        let s_3_118: u32 = (s_3_117.value() as u32);
        // S s_3_119: call DLR_write(s_3_118)
        let s_3_119: () = DLR_write(state, tracer, s_3_118);
        // C s_3_120: const #32s : i64
        let s_3_120: i64 = 32;
        // C s_3_121: cast zx s_3_120 -> i
        let s_3_121: i128 = (i128::try_from(s_3_120).unwrap());
        // S s_3_122: call __UNKNOWN_bits(s_3_121)
        let s_3_122: Bits = u__UNKNOWN_bits(state, tracer, s_3_121);
        // S s_3_123: cast reint s_3_122 -> u32
        let s_3_123: u32 = (s_3_122.value() as u32);
        // S s_3_124: call Mk_DSPSR_Type(s_3_123)
        let s_3_124: ProductType700c18a878c5601b = Mk_DSPSR_Type(state, tracer, s_3_123);
        // S s_3_125: call DSPSR_write(s_3_124)
        let s_3_125: () = DSPSR_write(state, tracer, s_3_124);
        // N s_3_126: jump b2
        return block_2(state, tracer, fn_state);
    }
}
