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
use AArch64_AllocationTagFromAddress::*;
use AArch64_IsUnprivAccessPriv::*;
use SETGOptionA::*;
use SPESampleMemSet::*;
use SETPreSizeChoice::*;
use IsAligned__1::*;
use X_read::*;
use Zeros::*;
use SETSizeChoice::*;
use Mem_set::*;
use neq_int::*;
use X_set::*;
use u__id::*;
use AArch64_MemTag_set::*;
use MemSetZeroSizeCheck::*;
use integer_subrange::*;
use replicate_bits_borealis_internal::*;
use SETPostSizeChoice::*;
use AArch64_Abort::*;
use MemSetParametersIllformedM::*;
use CreateAccDescSTGMOPS::*;
use MemSetParametersIllformedE::*;
use MismatchedMemSetException::*;
use AlignmentFault::*;
use fdiv_int::*;
use common::*;
pub fn execute_aarch64_instrs_memory_mcpymset_setg<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
    options_name: u8,
    s: i64,
    stage: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        tag: u8,
        toaddress: u64,
        setsize: u64,
        gs_168087: bool,
        tagaddr: u64,
        accdesc: ProductType9878976b5bcce9c9,
        stagesetsize: u64,
        B: i128,
        gs_168054: bool,
        postsize: u64,
        gs_168081: bool,
        nzcv: u8,
        tagstep: i128,
        gs_168062: bool,
        supports_option_a: bool,
        data: u8,
        privileged: bool,
        gs_168060: bool,
        gs_168066: bool,
        d: i64,
        n: i64,
        options_name: u8,
        s: i64,
        stage: u32,
    }
    let fn_state = FunctionState {
        d,
        n,
        options_name,
        s,
        stage,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #64s : i64
        let s_0_0: i64 = 64;
        // D s_0_1: read-var d:i64
        let s_0_1: i64 = fn_state.d;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: call X_read(s_0_2, s_0_0)
        let s_0_3: Bits = X_read(state, tracer, s_0_2, s_0_0);
        // D s_0_4: cast reint s_0_3 -> u64
        let s_0_4: u64 = (s_0_3.value() as u64);
        // D s_0_5: write-var toaddress <= s_0_4
        fn_state.toaddress = s_0_4;
        // C s_0_6: const #64s : i64
        let s_0_6: i64 = 64;
        // D s_0_7: read-var n:i64
        let s_0_7: i64 = fn_state.n;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: call X_read(s_0_8, s_0_6)
        let s_0_9: Bits = X_read(state, tracer, s_0_8, s_0_6);
        // D s_0_10: cast reint s_0_9 -> u64
        let s_0_10: u64 = (s_0_9.value() as u64);
        // D s_0_11: write-var setsize <= s_0_10
        fn_state.setsize = s_0_10;
        // C s_0_12: const #8s : i64
        let s_0_12: i64 = 8;
        // D s_0_13: read-var s:i64
        let s_0_13: i64 = fn_state.s;
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_15: call X_read(s_0_14, s_0_12)
        let s_0_15: Bits = X_read(state, tracer, s_0_14, s_0_12);
        // D s_0_16: cast reint s_0_15 -> u8
        let s_0_16: u8 = (s_0_15.value() as u8);
        // D s_0_17: write-var data <= s_0_16
        fn_state.data = s_0_16;
        // C s_0_18: const #16984u : u32
        let s_0_18: u32 = 16984;
        // D s_0_19: read-reg s_0_18:u8
        let s_0_19: bool = {
            let value = state.read_register::<bool>(s_0_18 as isize);
            tracer.read_register(s_0_18 as isize, value);
            value
        };
        // C s_0_20: const #16997u : u32
        let s_0_20: u32 = 16997;
        // D s_0_21: read-reg s_0_20:u8
        let s_0_21: bool = {
            let value = state.read_register::<bool>(s_0_20 as isize);
            tracer.read_register(s_0_20 as isize, value);
            value
        };
        // C s_0_22: const #16971u : u32
        let s_0_22: u32 = 16971;
        // D s_0_23: read-reg s_0_22:u8
        let s_0_23: bool = {
            let value = state.read_register::<bool>(s_0_22 as isize);
            tracer.read_register(s_0_22 as isize, value);
            value
        };
        // C s_0_24: const #16996u : u32
        let s_0_24: u32 = 16996;
        // D s_0_25: read-reg s_0_24:u8
        let s_0_25: bool = {
            let value = state.read_register::<bool>(s_0_24 as isize);
            tracer.read_register(s_0_24 as isize, value);
            value
        };
        // D s_0_26: cast zx s_0_23 -> bv
        let s_0_26: Bits = Bits::new(s_0_23 as u128, 1u16);
        // D s_0_27: cast zx s_0_25 -> bv
        let s_0_27: Bits = Bits::new(s_0_25 as u128, 1u16);
        // D s_0_28: cast reint s_0_26 -> u128
        let s_0_28: u128 = (s_0_26.value() as u128);
        // D s_0_29: size-of s_0_26
        let s_0_29: u16 = s_0_26.length();
        // D s_0_30: cast reint s_0_27 -> u128
        let s_0_30: u128 = (s_0_27.value() as u128);
        // D s_0_31: size-of s_0_27
        let s_0_31: u16 = s_0_27.length();
        // D s_0_32: lsl s_0_28 s_0_31
        let s_0_32: u128 = s_0_28 << s_0_31;
        // D s_0_33: or s_0_32 s_0_30
        let s_0_33: u128 = ((s_0_32) | (s_0_30));
        // D s_0_34: add s_0_29 s_0_31
        let s_0_34: u16 = (s_0_29 + s_0_31);
        // D s_0_35: create-bits s_0_33 s_0_34
        let s_0_35: Bits = Bits::new(s_0_33, s_0_34);
        // D s_0_36: cast reint s_0_35 -> u8
        let s_0_36: u8 = (s_0_35.value() as u8);
        // D s_0_37: cast zx s_0_21 -> bv
        let s_0_37: Bits = Bits::new(s_0_21 as u128, 1u16);
        // D s_0_38: cast zx s_0_36 -> bv
        let s_0_38: Bits = Bits::new(s_0_36 as u128, 2u16);
        // D s_0_39: cast reint s_0_37 -> u128
        let s_0_39: u128 = (s_0_37.value() as u128);
        // D s_0_40: size-of s_0_37
        let s_0_40: u16 = s_0_37.length();
        // D s_0_41: cast reint s_0_38 -> u128
        let s_0_41: u128 = (s_0_38.value() as u128);
        // D s_0_42: size-of s_0_38
        let s_0_42: u16 = s_0_38.length();
        // D s_0_43: lsl s_0_39 s_0_42
        let s_0_43: u128 = s_0_39 << s_0_42;
        // D s_0_44: or s_0_43 s_0_41
        let s_0_44: u128 = ((s_0_43) | (s_0_41));
        // D s_0_45: add s_0_40 s_0_42
        let s_0_45: u16 = (s_0_40 + s_0_42);
        // D s_0_46: create-bits s_0_44 s_0_45
        let s_0_46: Bits = Bits::new(s_0_44, s_0_45);
        // D s_0_47: cast reint s_0_46 -> u8
        let s_0_47: u8 = (s_0_46.value() as u8);
        // D s_0_48: cast zx s_0_19 -> bv
        let s_0_48: Bits = Bits::new(s_0_19 as u128, 1u16);
        // D s_0_49: cast zx s_0_47 -> bv
        let s_0_49: Bits = Bits::new(s_0_47 as u128, 3u16);
        // D s_0_50: cast reint s_0_48 -> u128
        let s_0_50: u128 = (s_0_48.value() as u128);
        // D s_0_51: size-of s_0_48
        let s_0_51: u16 = s_0_48.length();
        // D s_0_52: cast reint s_0_49 -> u128
        let s_0_52: u128 = (s_0_49.value() as u128);
        // D s_0_53: size-of s_0_49
        let s_0_53: u16 = s_0_49.length();
        // D s_0_54: lsl s_0_50 s_0_53
        let s_0_54: u128 = s_0_50 << s_0_53;
        // D s_0_55: or s_0_54 s_0_52
        let s_0_55: u128 = ((s_0_54) | (s_0_52));
        // D s_0_56: add s_0_51 s_0_53
        let s_0_56: u16 = (s_0_51 + s_0_53);
        // D s_0_57: create-bits s_0_55 s_0_56
        let s_0_57: Bits = Bits::new(s_0_55, s_0_56);
        // D s_0_58: cast reint s_0_57 -> u8
        let s_0_58: u8 = (s_0_57.value() as u8);
        // D s_0_59: write-var nzcv <= s_0_58
        fn_state.nzcv = s_0_58;
        // C s_0_60: const #() : ()
        let s_0_60: () = ();
        // S s_0_61: call SETGOptionA(s_0_60)
        let s_0_61: bool = SETGOptionA(state, tracer, s_0_60);
        // D s_0_62: write-var supports_option_a <= s_0_61
        fn_state.supports_option_a = s_0_61;
        // C s_0_63: const #0s : i
        let s_0_63: i128 = 0;
        // D s_0_64: read-var options_name:u8
        let s_0_64: u8 = fn_state.options_name;
        // D s_0_65: cast zx s_0_64 -> bv
        let s_0_65: Bits = Bits::new(s_0_64 as u128, 2u16);
        // C s_0_66: const #1u : u64
        let s_0_66: u64 = 1;
        // D s_0_67: bit-extract s_0_65 s_0_63 s_0_66
        let s_0_67: Bits = (Bits::new(
            ((s_0_65) >> (s_0_63)).value(),
            u16::try_from(s_0_66).unwrap(),
        ));
        // D s_0_68: cast reint s_0_67 -> u8
        let s_0_68: bool = ((s_0_67.value()) != 0);
        // C s_0_69: const #0s : i
        let s_0_69: i128 = 0;
        // C s_0_70: const #0u : u64
        let s_0_70: u64 = 0;
        // D s_0_71: cast zx s_0_68 -> u64
        let s_0_71: u64 = (s_0_68 as u64);
        // C s_0_72: const #1u : u64
        let s_0_72: u64 = 1;
        // D s_0_73: and s_0_71 s_0_72
        let s_0_73: u64 = ((s_0_71) & (s_0_72));
        // D s_0_74: cmp-eq s_0_73 s_0_72
        let s_0_74: bool = ((s_0_73) == (s_0_72));
        // D s_0_75: lsl s_0_71 s_0_69
        let s_0_75: u64 = s_0_71 << s_0_69;
        // D s_0_76: or s_0_70 s_0_75
        let s_0_76: u64 = ((s_0_70) | (s_0_75));
        // D s_0_77: cmpl s_0_75
        let s_0_77: u64 = !s_0_75;
        // D s_0_78: and s_0_70 s_0_77
        let s_0_78: u64 = ((s_0_70) & (s_0_77));
        // D s_0_79: select s_0_74 s_0_76 s_0_78
        let s_0_79: u64 = if s_0_74 { s_0_76 } else { s_0_78 };
        // D s_0_80: cast trunc s_0_79 -> u8
        let s_0_80: bool = ((s_0_79) != 0);
        // D s_0_81: cast zx s_0_80 -> bv
        let s_0_81: Bits = Bits::new(s_0_80 as u128, 1u16);
        // C s_0_82: const #1u : u8
        let s_0_82: bool = true;
        // C s_0_83: cast zx s_0_82 -> bv
        let s_0_83: Bits = Bits::new(s_0_82 as u128, 1u16);
        // D s_0_84: cmp-eq s_0_81 s_0_83
        let s_0_84: bool = ((s_0_81) == (s_0_83));
        // N s_0_85: branch s_0_84 b94 b1
        if s_0_84 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #448u : u32
        let s_1_3: u32 = 448;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-ne s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) != (s_1_5));
        // D s_1_7: write-var privileged <= s_1_6
        fn_state.privileged = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #1s : i
        let s_2_0: i128 = 1;
        // D s_2_1: read-var options_name:u8
        let s_2_1: u8 = fn_state.options_name;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 2u16);
        // C s_2_3: const #1u : u64
        let s_2_3: u64 = 1;
        // D s_2_4: bit-extract s_2_2 s_2_0 s_2_3
        let s_2_4: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #0u : u64
        let s_2_7: u64 = 0;
        // D s_2_8: cast zx s_2_5 -> u64
        let s_2_8: u64 = (s_2_5 as u64);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: and s_2_8 s_2_9
        let s_2_10: u64 = ((s_2_8) & (s_2_9));
        // D s_2_11: cmp-eq s_2_10 s_2_9
        let s_2_11: bool = ((s_2_10) == (s_2_9));
        // D s_2_12: lsl s_2_8 s_2_6
        let s_2_12: u64 = s_2_8 << s_2_6;
        // D s_2_13: or s_2_7 s_2_12
        let s_2_13: u64 = ((s_2_7) | (s_2_12));
        // D s_2_14: cmpl s_2_12
        let s_2_14: u64 = !s_2_12;
        // D s_2_15: and s_2_7 s_2_14
        let s_2_15: u64 = ((s_2_7) & (s_2_14));
        // D s_2_16: select s_2_11 s_2_13 s_2_15
        let s_2_16: u64 = if s_2_11 { s_2_13 } else { s_2_15 };
        // D s_2_17: cast trunc s_2_16 -> u8
        let s_2_17: bool = ((s_2_16) != 0);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // C s_2_19: const #1u : u8
        let s_2_19: bool = true;
        // C s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_21: cmp-eq s_2_18 s_2_20
        let s_2_21: bool = ((s_2_18) == (s_2_20));
        // D s_2_22: read-var privileged:u8
        let s_2_22: bool = fn_state.privileged;
        // D s_2_23: call CreateAccDescSTGMOPS(s_2_22, s_2_21)
        let s_2_23: ProductType9878976b5bcce9c9 = CreateAccDescSTGMOPS(
            state,
            tracer,
            s_2_22,
            s_2_21,
        );
        // D s_2_24: write-var accdesc <= s_2_23
        fn_state.accdesc = s_2_23;
        // D s_2_25: read-var stage:u32
        let s_2_25: u32 = fn_state.stage;
        // C s_2_26: const #0u : u32
        let s_2_26: u32 = 0;
        // D s_2_27: cmp-eq s_2_25 s_2_26
        let s_2_27: bool = ((s_2_25) == (s_2_26));
        // N s_2_28: branch s_2_27 b73 b3
        if s_2_27 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var toaddress:u64
        let s_3_0: u64 = fn_state.toaddress;
        // D s_3_1: read-var setsize:u64
        let s_3_1: u64 = fn_state.setsize;
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // D s_3_3: call SETPostSizeChoice(s_3_0, s_3_1, s_3_2)
        let s_3_3: u64 = SETPostSizeChoice(state, tracer, s_3_0, s_3_1, s_3_2);
        // D s_3_4: write-var postsize <= s_3_3
        fn_state.postsize = s_3_3;
        // C s_3_5: const #63s : i
        let s_3_5: i128 = 63;
        // D s_3_6: read-var postsize:u64
        let s_3_6: u64 = fn_state.postsize;
        // D s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 64u16);
        // C s_3_8: const #1u : u64
        let s_3_8: u64 = 1;
        // D s_3_9: bit-extract s_3_7 s_3_5 s_3_8
        let s_3_9: Bits = (Bits::new(
            ((s_3_7) >> (s_3_5)).value(),
            u16::try_from(s_3_8).unwrap(),
        ));
        // D s_3_10: cast reint s_3_9 -> u8
        let s_3_10: bool = ((s_3_9.value()) != 0);
        // C s_3_11: const #0s : i
        let s_3_11: i128 = 0;
        // C s_3_12: const #0u : u64
        let s_3_12: u64 = 0;
        // D s_3_13: cast zx s_3_10 -> u64
        let s_3_13: u64 = (s_3_10 as u64);
        // C s_3_14: const #1u : u64
        let s_3_14: u64 = 1;
        // D s_3_15: and s_3_13 s_3_14
        let s_3_15: u64 = ((s_3_13) & (s_3_14));
        // D s_3_16: cmp-eq s_3_15 s_3_14
        let s_3_16: bool = ((s_3_15) == (s_3_14));
        // D s_3_17: lsl s_3_13 s_3_11
        let s_3_17: u64 = s_3_13 << s_3_11;
        // D s_3_18: or s_3_12 s_3_17
        let s_3_18: u64 = ((s_3_12) | (s_3_17));
        // D s_3_19: cmpl s_3_17
        let s_3_19: u64 = !s_3_17;
        // D s_3_20: and s_3_12 s_3_19
        let s_3_20: u64 = ((s_3_12) & (s_3_19));
        // D s_3_21: select s_3_16 s_3_18 s_3_20
        let s_3_21: u64 = if s_3_16 { s_3_18 } else { s_3_20 };
        // D s_3_22: cast trunc s_3_21 -> u8
        let s_3_22: bool = ((s_3_21) != 0);
        // C s_3_23: const #63s : i
        let s_3_23: i128 = 63;
        // D s_3_24: read-var setsize:u64
        let s_3_24: u64 = fn_state.setsize;
        // D s_3_25: cast zx s_3_24 -> bv
        let s_3_25: Bits = Bits::new(s_3_24 as u128, 64u16);
        // C s_3_26: const #1u : u64
        let s_3_26: u64 = 1;
        // D s_3_27: bit-extract s_3_25 s_3_23 s_3_26
        let s_3_27: Bits = (Bits::new(
            ((s_3_25) >> (s_3_23)).value(),
            u16::try_from(s_3_26).unwrap(),
        ));
        // D s_3_28: cast reint s_3_27 -> u8
        let s_3_28: bool = ((s_3_27.value()) != 0);
        // C s_3_29: const #0s : i
        let s_3_29: i128 = 0;
        // C s_3_30: const #0u : u64
        let s_3_30: u64 = 0;
        // D s_3_31: cast zx s_3_28 -> u64
        let s_3_31: u64 = (s_3_28 as u64);
        // C s_3_32: const #1u : u64
        let s_3_32: u64 = 1;
        // D s_3_33: and s_3_31 s_3_32
        let s_3_33: u64 = ((s_3_31) & (s_3_32));
        // D s_3_34: cmp-eq s_3_33 s_3_32
        let s_3_34: bool = ((s_3_33) == (s_3_32));
        // D s_3_35: lsl s_3_31 s_3_29
        let s_3_35: u64 = s_3_31 << s_3_29;
        // D s_3_36: or s_3_30 s_3_35
        let s_3_36: u64 = ((s_3_30) | (s_3_35));
        // D s_3_37: cmpl s_3_35
        let s_3_37: u64 = !s_3_35;
        // D s_3_38: and s_3_30 s_3_37
        let s_3_38: u64 = ((s_3_30) & (s_3_37));
        // D s_3_39: select s_3_34 s_3_36 s_3_38
        let s_3_39: u64 = if s_3_34 { s_3_36 } else { s_3_38 };
        // D s_3_40: cast trunc s_3_39 -> u8
        let s_3_40: bool = ((s_3_39) != 0);
        // D s_3_41: cast zx s_3_22 -> bv
        let s_3_41: Bits = Bits::new(s_3_22 as u128, 1u16);
        // D s_3_42: cast zx s_3_40 -> bv
        let s_3_42: Bits = Bits::new(s_3_40 as u128, 1u16);
        // D s_3_43: cmp-eq s_3_41 s_3_42
        let s_3_43: bool = ((s_3_41) == (s_3_42));
        // N s_3_44: branch s_3_43 b72 b4
        if s_3_43 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64s : i
        let s_4_0: i128 = 64;
        // S s_4_1: call Zeros(s_4_0)
        let s_4_1: Bits = Zeros(state, tracer, s_4_0);
        // S s_4_2: cast reint s_4_1 -> u64
        let s_4_2: u64 = (s_4_1.value() as u64);
        // D s_4_3: read-var postsize:u64
        let s_4_3: u64 = fn_state.postsize;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 64u16);
        // S s_4_5: cast zx s_4_2 -> bv
        let s_4_5: Bits = Bits::new(s_4_2 as u128, 64u16);
        // D s_4_6: cmp-eq s_4_4 s_4_5
        let s_4_6: bool = ((s_4_4) == (s_4_5));
        // D s_4_7: write-var gs#168054 <= s_4_6
        fn_state.gs_168054 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#168054:u8
        let s_5_0: bool = fn_state.gs_168054;
        // N s_5_1: assert s_5_0
        let s_5_1: () = assert!(s_5_0);
        // C s_5_2: const #0s : i
        let s_5_2: i128 = 0;
        // D s_5_3: read-var postsize:u64
        let s_5_3: u64 = fn_state.postsize;
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 64u16);
        // C s_5_5: const #1s : i64
        let s_5_5: i64 = 1;
        // C s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // C s_5_7: const #3s : i
        let s_5_7: i128 = 3;
        // C s_5_8: add s_5_7 s_5_6
        let s_5_8: i128 = (s_5_7 + s_5_6);
        // D s_5_9: bit-extract s_5_4 s_5_2 s_5_8
        let s_5_9: Bits = (Bits::new(
            ((s_5_4) >> (s_5_2)).value(),
            u16::try_from(s_5_8).unwrap(),
        ));
        // D s_5_10: cast reint s_5_9 -> u8
        let s_5_10: u8 = (s_5_9.value() as u8);
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 4u16);
        // C s_5_12: const #0u : u8
        let s_5_12: u8 = 0;
        // C s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 4u16);
        // D s_5_14: cmp-eq s_5_11 s_5_13
        let s_5_14: bool = ((s_5_11) == (s_5_13));
        // N s_5_15: assert s_5_14
        let s_5_15: () = assert!(s_5_14);
        // C s_5_16: const #() : ()
        let s_5_16: () = ();
        // S s_5_17: call MemSetZeroSizeCheck(s_5_16)
        let s_5_17: bool = MemSetZeroSizeCheck(state, tracer, s_5_16);
        // N s_5_18: branch s_5_17 b71 b6
        if s_5_17 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var setsize:u64
        let s_6_0: u64 = fn_state.setsize;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 64u16);
        // D s_6_2: cast sx s_6_1 -> i
        let s_6_2: i128 = {
            let sign_bit = s_6_1.length() - 1;
            let mut result = s_6_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #0s : i
        let s_6_4: i128 = 0;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: call neq_int(s_6_5, s_6_4)
        let s_6_6: bool = neq_int(state, tracer, s_6_5, s_6_4);
        // D s_6_7: write-var gs#168060 <= s_6_6
        fn_state.gs_168060 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#168060:u8
        let s_7_0: bool = fn_state.gs_168060;
        // N s_7_1: branch s_7_0 b62 b8
        if s_7_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var stage:u32
        let s_9_0: u32 = fn_state.stage;
        // C s_9_1: const #1u : u32
        let s_9_1: u32 = 1;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // N s_9_3: branch s_9_2 b58 b10
        if s_9_2 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var postsize:u64
        let s_10_0: u64 = fn_state.postsize;
        // D s_10_1: write-var stagesetsize <= s_10_0
        fn_state.stagesetsize = s_10_0;
        // D s_10_2: read-var setsize:u64
        let s_10_2: u64 = fn_state.setsize;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 64u16);
        // D s_10_4: read-var postsize:u64
        let s_10_4: u64 = fn_state.postsize;
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 64u16);
        // D s_10_6: cmp-ne s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) != (s_10_5));
        // N s_10_7: branch s_10_6 b57 b11
        if s_10_6 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var toaddress:u64
        let s_11_0: u64 = fn_state.toaddress;
        // D s_11_1: read-var setsize:u64
        let s_11_1: u64 = fn_state.setsize;
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // D s_11_3: call MemSetParametersIllformedE(s_11_0, s_11_1, s_11_2)
        let s_11_3: bool = MemSetParametersIllformedE(
            state,
            tracer,
            s_11_0,
            s_11_1,
            s_11_2,
        );
        // D s_11_4: write-var gs#168066 <= s_11_3
        fn_state.gs_168066 = s_11_3;
        // N s_11_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#168066:u8
        let s_12_0: bool = fn_state.gs_168066;
        // N s_12_1: branch s_12_0 b56 b13
        if s_12_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #64s : i
        let s_15_0: i128 = 64;
        // S s_15_1: call Zeros(s_15_0)
        let s_15_1: Bits = Zeros(state, tracer, s_15_0);
        // S s_15_2: cast reint s_15_1 -> u64
        let s_15_2: u64 = (s_15_1.value() as u64);
        // D s_15_3: read-var setsize:u64
        let s_15_3: u64 = fn_state.setsize;
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 64u16);
        // S s_15_5: cast zx s_15_2 -> bv
        let s_15_5: Bits = Bits::new(s_15_2 as u128, 64u16);
        // D s_15_6: cmp-ne s_15_4 s_15_5
        let s_15_6: bool = ((s_15_4) != (s_15_5));
        // N s_15_7: branch s_15_6 b55 b16
        if s_15_6 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#168062 <= s_16_0
        fn_state.gs_168062 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#168062:u8
        let s_17_0: bool = fn_state.gs_168062;
        // N s_17_1: branch s_17_0 b54 b18
        if s_17_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var setsize:u64
        let s_19_0: u64 = fn_state.setsize;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 64u16);
        // C s_19_2: const #1344u : u32
        let s_19_2: u32 = 1344;
        // D s_19_3: read-reg s_19_2:i64
        let s_19_3: i64 = {
            let value = state.read_register::<i64>(s_19_2 as isize);
            tracer.read_register(s_19_2 as isize, value);
            value
        };
        // D s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (i128::try_from(s_19_3).unwrap());
        // D s_19_5: call IsAligned__1(s_19_1, s_19_4)
        let s_19_5: bool = IsAligned__1(state, tracer, s_19_1, s_19_4);
        // D s_19_6: not s_19_5
        let s_19_6: bool = !s_19_5;
        // N s_19_7: branch s_19_6 b53 b20
        if s_19_6 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
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
        // C s_22_0: const #22416u : u32
        let s_22_0: u32 = 22416;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: bool = {
            let value = state.read_register::<bool>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // N s_22_2: branch s_22_1 b52 b23
        if s_22_1 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var supports_option_a:u8
        let s_24_0: bool = fn_state.supports_option_a;
        // N s_24_1: branch s_24_0 b40 b25
        if s_24_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var stagesetsize:u64
        let s_26_0: u64 = fn_state.stagesetsize;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 64u16);
        // D s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: const #0s : i
        let s_26_3: i128 = 0;
        // D s_26_4: cmp-gt s_26_2 s_26_3
        let s_26_4: bool = ((s_26_2) > (s_26_3));
        // D s_26_5: not s_26_4
        let s_26_5: bool = !s_26_4;
        // N s_26_6: branch s_26_5 b36 b27
        if s_26_5 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #16s : i
        let s_27_0: i128 = 16;
        // D s_27_1: read-var toaddress:u64
        let s_27_1: u64 = fn_state.toaddress;
        // D s_27_2: read-var setsize:u64
        let s_27_2: u64 = fn_state.setsize;
        // D s_27_3: call SETSizeChoice(s_27_1, s_27_2, s_27_0)
        let s_27_3: i128 = SETSizeChoice(state, tracer, s_27_1, s_27_2, s_27_0);
        // D s_27_4: write-var B <= s_27_3
        fn_state.B = s_27_3;
        // D s_27_5: read-var stagesetsize:u64
        let s_27_5: u64 = fn_state.stagesetsize;
        // D s_27_6: cast zx s_27_5 -> bv
        let s_27_6: Bits = Bits::new(s_27_5 as u128, 64u16);
        // D s_27_7: cast zx s_27_6 -> i
        let s_27_7: i128 = (s_27_6.value() as i128);
        // D s_27_8: read-var B:i
        let s_27_8: i128 = fn_state.B;
        // D s_27_9: cmp-le s_27_8 s_27_7
        let s_27_9: bool = ((s_27_8) <= (s_27_7));
        // N s_27_10: assert s_27_9
        let s_27_10: () = assert!(s_27_9);
        // C s_27_11: const #3s : i
        let s_27_11: i128 = 3;
        // C s_27_12: const #0s : i
        let s_27_12: i128 = 0;
        // D s_27_13: read-var B:i
        let s_27_13: i128 = fn_state.B;
        // D s_27_14: call integer_subrange(s_27_13, s_27_11, s_27_12)
        let s_27_14: Bits = integer_subrange(state, tracer, s_27_13, s_27_11, s_27_12);
        // D s_27_15: cast reint s_27_14 -> u8
        let s_27_15: u8 = (s_27_14.value() as u8);
        // D s_27_16: cast zx s_27_15 -> bv
        let s_27_16: Bits = Bits::new(s_27_15 as u128, 4u16);
        // C s_27_17: const #0u : u8
        let s_27_17: u8 = 0;
        // C s_27_18: cast zx s_27_17 -> bv
        let s_27_18: Bits = Bits::new(s_27_17 as u128, 4u16);
        // D s_27_19: cmp-eq s_27_16 s_27_18
        let s_27_19: bool = ((s_27_16) == (s_27_18));
        // N s_27_20: assert s_27_19
        let s_27_20: () = assert!(s_27_19);
        // D s_27_21: read-var B:i
        let s_27_21: i128 = fn_state.B;
        // D s_27_22: call __id(s_27_21)
        let s_27_22: i128 = u__id(state, tracer, s_27_21);
        // C s_27_23: const #8s : i
        let s_27_23: i128 = 8;
        // D s_27_24: mul s_27_22 s_27_23
        let s_27_24: i128 = ((s_27_22) * (s_27_23));
        // C s_27_25: const #0s : i
        let s_27_25: i128 = 0;
        // D s_27_26: cmp-ge s_27_24 s_27_25
        let s_27_26: bool = ((s_27_24) >= (s_27_25));
        // N s_27_27: assert s_27_26
        let s_27_27: () = assert!(s_27_26);
        // D s_27_28: read-var data:u8
        let s_27_28: u8 = fn_state.data;
        // D s_27_29: cast zx s_27_28 -> bv
        let s_27_29: Bits = Bits::new(s_27_28 as u128, 8u16);
        // D s_27_30: cast reint s_27_21 -> u64
        let s_27_30: u64 = (s_27_21 as u64);
        // D s_27_31: call replicate_bits_borealis_internal(s_27_29, s_27_30)
        let s_27_31: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_27_29,
            s_27_30,
        );
        // D s_27_32: read-var toaddress:u64
        let s_27_32: u64 = fn_state.toaddress;
        // D s_27_33: read-var accdesc:struct
        let s_27_33: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_27_34: call Mem_set(s_27_32, s_27_21, s_27_33, s_27_31)
        let s_27_34: () = Mem_set(state, tracer, s_27_32, s_27_21, s_27_33, s_27_31);
        // N s_27_35: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #16s : i
        let s_28_0: i128 = 16;
        // D s_28_1: read-var B:i
        let s_28_1: i128 = fn_state.B;
        // D s_28_2: call fdiv_int(s_28_1, s_28_0)
        let s_28_2: i128 = fdiv_int(state, tracer, s_28_1, s_28_0);
        // D s_28_3: write-var tagstep <= s_28_2
        fn_state.tagstep = s_28_2;
        // D s_28_4: read-var toaddress:u64
        let s_28_4: u64 = fn_state.toaddress;
        // D s_28_5: call AArch64_AllocationTagFromAddress(s_28_4)
        let s_28_5: u8 = AArch64_AllocationTagFromAddress(state, tracer, s_28_4);
        // D s_28_6: write-var tag <= s_28_5
        fn_state.tag = s_28_5;
        // N s_28_7: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0s : i
        let s_29_0: i128 = 0;
        // D s_29_1: read-var tagstep:i
        let s_29_1: i128 = fn_state.tagstep;
        // D s_29_2: cmp-gt s_29_1 s_29_0
        let s_29_2: bool = ((s_29_1) > (s_29_0));
        // D s_29_3: not s_29_2
        let s_29_3: bool = !s_29_2;
        // N s_29_4: branch s_29_3 b32 b30
        if s_29_3 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1s : i
        let s_30_0: i128 = 1;
        // D s_30_1: read-var tagstep:i
        let s_30_1: i128 = fn_state.tagstep;
        // D s_30_2: sub s_30_1 s_30_0
        let s_30_2: i128 = ((s_30_1) - (s_30_0));
        // C s_30_3: const #16s : i
        let s_30_3: i128 = 16;
        // D s_30_4: mul s_30_2 s_30_3
        let s_30_4: i128 = ((s_30_2) * (s_30_3));
        // D s_30_5: read-var toaddress:u64
        let s_30_5: u64 = fn_state.toaddress;
        // D s_30_6: cast zx s_30_5 -> bv
        let s_30_6: Bits = Bits::new(s_30_5 as u128, 64u16);
        // D s_30_7: cast cvt s_30_4 -> bv
        let s_30_7: Bits = Bits::new(s_30_4 as u128, 128);
        // D s_30_8: add s_30_6 s_30_7
        let s_30_8: Bits = (s_30_6 + s_30_7);
        // D s_30_9: cast reint s_30_8 -> u64
        let s_30_9: u64 = (s_30_8.value() as u64);
        // D s_30_10: write-var tagaddr <= s_30_9
        fn_state.tagaddr = s_30_9;
        // D s_30_11: read-var tagaddr:u64
        let s_30_11: u64 = fn_state.tagaddr;
        // D s_30_12: read-var accdesc:struct
        let s_30_12: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_30_13: read-var tag:u8
        let s_30_13: u8 = fn_state.tag;
        // D s_30_14: call AArch64_MemTag_set(s_30_11, s_30_12, s_30_13)
        let s_30_14: () = AArch64_MemTag_set(state, tracer, s_30_11, s_30_12, s_30_13);
        // N s_30_15: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1s : i
        let s_31_0: i128 = 1;
        // D s_31_1: read-var tagstep:i
        let s_31_1: i128 = fn_state.tagstep;
        // D s_31_2: sub s_31_1 s_31_0
        let s_31_2: i128 = ((s_31_1) - (s_31_0));
        // D s_31_3: write-var tagstep <= s_31_2
        fn_state.tagstep = s_31_2;
        // N s_31_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var toaddress:u64
        let s_32_0: u64 = fn_state.toaddress;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 64u16);
        // D s_32_2: read-var B:i
        let s_32_2: i128 = fn_state.B;
        // D s_32_3: cast cvt s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 128);
        // D s_32_4: add s_32_1 s_32_3
        let s_32_4: Bits = (s_32_1 + s_32_3);
        // D s_32_5: cast reint s_32_4 -> u64
        let s_32_5: u64 = (s_32_4.value() as u64);
        // D s_32_6: write-var toaddress <= s_32_5
        fn_state.toaddress = s_32_5;
        // D s_32_7: read-var setsize:u64
        let s_32_7: u64 = fn_state.setsize;
        // D s_32_8: cast zx s_32_7 -> bv
        let s_32_8: Bits = Bits::new(s_32_7 as u128, 64u16);
        // D s_32_9: read-var B:i
        let s_32_9: i128 = fn_state.B;
        // D s_32_10: cast cvt s_32_9 -> bv
        let s_32_10: Bits = Bits::new(s_32_9 as u128, 128);
        // D s_32_11: sub s_32_8 s_32_10
        let s_32_11: Bits = ((s_32_8) - (s_32_10));
        // D s_32_12: cast reint s_32_11 -> u64
        let s_32_12: u64 = (s_32_11.value() as u64);
        // D s_32_13: write-var setsize <= s_32_12
        fn_state.setsize = s_32_12;
        // D s_32_14: read-var stagesetsize:u64
        let s_32_14: u64 = fn_state.stagesetsize;
        // D s_32_15: cast zx s_32_14 -> bv
        let s_32_15: Bits = Bits::new(s_32_14 as u128, 64u16);
        // D s_32_16: read-var B:i
        let s_32_16: i128 = fn_state.B;
        // D s_32_17: cast cvt s_32_16 -> bv
        let s_32_17: Bits = Bits::new(s_32_16 as u128, 128);
        // D s_32_18: sub s_32_15 s_32_17
        let s_32_18: Bits = ((s_32_15) - (s_32_17));
        // D s_32_19: cast reint s_32_18 -> u64
        let s_32_19: u64 = (s_32_18.value() as u64);
        // D s_32_20: write-var stagesetsize <= s_32_19
        fn_state.stagesetsize = s_32_19;
        // D s_32_21: read-var stage:u32
        let s_32_21: u32 = fn_state.stage;
        // C s_32_22: const #0u : u32
        let s_32_22: u32 = 0;
        // D s_32_23: cmp-eq s_32_21 s_32_22
        let s_32_23: bool = ((s_32_21) == (s_32_22));
        // N s_32_24: branch s_32_23 b35 b33
        if s_32_23 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #64s : i64
        let s_35_0: i64 = 64;
        // D s_35_1: read-var n:i64
        let s_35_1: i64 = fn_state.n;
        // D s_35_2: cast zx s_35_1 -> i
        let s_35_2: i128 = (i128::try_from(s_35_1).unwrap());
        // D s_35_3: read-var setsize:u64
        let s_35_3: u64 = fn_state.setsize;
        // D s_35_4: cast zx s_35_3 -> bv
        let s_35_4: Bits = Bits::new(s_35_3 as u128, 64u16);
        // D s_35_5: call X_set(s_35_2, s_35_0, s_35_4)
        let s_35_5: () = X_set(state, tracer, s_35_2, s_35_0, s_35_4);
        // C s_35_6: const #64s : i64
        let s_35_6: i64 = 64;
        // D s_35_7: read-var d:i64
        let s_35_7: i64 = fn_state.d;
        // D s_35_8: cast zx s_35_7 -> i
        let s_35_8: i128 = (i128::try_from(s_35_7).unwrap());
        // D s_35_9: read-var toaddress:u64
        let s_35_9: u64 = fn_state.toaddress;
        // D s_35_10: cast zx s_35_9 -> bv
        let s_35_10: Bits = Bits::new(s_35_9 as u128, 64u16);
        // D s_35_11: call X_set(s_35_8, s_35_6, s_35_10)
        let s_35_11: () = X_set(state, tracer, s_35_8, s_35_6, s_35_10);
        // N s_35_12: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var stage:u32
        let s_37_0: u32 = fn_state.stage;
        // C s_37_1: const #0u : u32
        let s_37_1: u32 = 0;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // N s_37_3: branch s_37_2 b39 b38
        if s_37_2 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #64s : i64
        let s_39_0: i64 = 64;
        // D s_39_1: read-var n:i64
        let s_39_1: i64 = fn_state.n;
        // D s_39_2: cast zx s_39_1 -> i
        let s_39_2: i128 = (i128::try_from(s_39_1).unwrap());
        // D s_39_3: read-var setsize:u64
        let s_39_3: u64 = fn_state.setsize;
        // D s_39_4: cast zx s_39_3 -> bv
        let s_39_4: Bits = Bits::new(s_39_3 as u128, 64u16);
        // D s_39_5: call X_set(s_39_2, s_39_0, s_39_4)
        let s_39_5: () = X_set(state, tracer, s_39_2, s_39_0, s_39_4);
        // C s_39_6: const #64s : i64
        let s_39_6: i64 = 64;
        // D s_39_7: read-var d:i64
        let s_39_7: i64 = fn_state.d;
        // D s_39_8: cast zx s_39_7 -> i
        let s_39_8: i128 = (i128::try_from(s_39_7).unwrap());
        // D s_39_9: read-var toaddress:u64
        let s_39_9: u64 = fn_state.toaddress;
        // D s_39_10: cast zx s_39_9 -> bv
        let s_39_10: Bits = Bits::new(s_39_9 as u128, 64u16);
        // D s_39_11: call X_set(s_39_8, s_39_6, s_39_10)
        let s_39_11: () = X_set(state, tracer, s_39_8, s_39_6, s_39_10);
        // C s_39_12: const #3s : i
        let s_39_12: i128 = 3;
        // D s_39_13: read-var nzcv:u8
        let s_39_13: u8 = fn_state.nzcv;
        // D s_39_14: cast zx s_39_13 -> bv
        let s_39_14: Bits = Bits::new(s_39_13 as u128, 4u16);
        // C s_39_15: const #1s : i64
        let s_39_15: i64 = 1;
        // C s_39_16: cast zx s_39_15 -> i
        let s_39_16: i128 = (i128::try_from(s_39_15).unwrap());
        // C s_39_17: const #0s : i
        let s_39_17: i128 = 0;
        // C s_39_18: add s_39_17 s_39_16
        let s_39_18: i128 = (s_39_17 + s_39_16);
        // D s_39_19: bit-extract s_39_14 s_39_12 s_39_18
        let s_39_19: Bits = (Bits::new(
            ((s_39_14) >> (s_39_12)).value(),
            u16::try_from(s_39_18).unwrap(),
        ));
        // D s_39_20: cast reint s_39_19 -> u8
        let s_39_20: bool = ((s_39_19.value()) != 0);
        // C s_39_21: const #16984u : u32
        let s_39_21: u32 = 16984;
        // N s_39_22: write-reg s_39_21 <= s_39_20
        let s_39_22: () = {
            state.write_register::<bool>(s_39_21 as isize, s_39_20);
            tracer.write_register(s_39_21 as isize, s_39_20);
        };
        // C s_39_23: const #2s : i
        let s_39_23: i128 = 2;
        // D s_39_24: read-var nzcv:u8
        let s_39_24: u8 = fn_state.nzcv;
        // D s_39_25: cast zx s_39_24 -> bv
        let s_39_25: Bits = Bits::new(s_39_24 as u128, 4u16);
        // C s_39_26: const #1s : i64
        let s_39_26: i64 = 1;
        // C s_39_27: cast zx s_39_26 -> i
        let s_39_27: i128 = (i128::try_from(s_39_26).unwrap());
        // C s_39_28: const #0s : i
        let s_39_28: i128 = 0;
        // C s_39_29: add s_39_28 s_39_27
        let s_39_29: i128 = (s_39_28 + s_39_27);
        // D s_39_30: bit-extract s_39_25 s_39_23 s_39_29
        let s_39_30: Bits = (Bits::new(
            ((s_39_25) >> (s_39_23)).value(),
            u16::try_from(s_39_29).unwrap(),
        ));
        // D s_39_31: cast reint s_39_30 -> u8
        let s_39_31: bool = ((s_39_30.value()) != 0);
        // C s_39_32: const #16997u : u32
        let s_39_32: u32 = 16997;
        // N s_39_33: write-reg s_39_32 <= s_39_31
        let s_39_33: () = {
            state.write_register::<bool>(s_39_32 as isize, s_39_31);
            tracer.write_register(s_39_32 as isize, s_39_31);
        };
        // C s_39_34: const #1s : i
        let s_39_34: i128 = 1;
        // D s_39_35: read-var nzcv:u8
        let s_39_35: u8 = fn_state.nzcv;
        // D s_39_36: cast zx s_39_35 -> bv
        let s_39_36: Bits = Bits::new(s_39_35 as u128, 4u16);
        // C s_39_37: const #1s : i64
        let s_39_37: i64 = 1;
        // C s_39_38: cast zx s_39_37 -> i
        let s_39_38: i128 = (i128::try_from(s_39_37).unwrap());
        // C s_39_39: const #0s : i
        let s_39_39: i128 = 0;
        // C s_39_40: add s_39_39 s_39_38
        let s_39_40: i128 = (s_39_39 + s_39_38);
        // D s_39_41: bit-extract s_39_36 s_39_34 s_39_40
        let s_39_41: Bits = (Bits::new(
            ((s_39_36) >> (s_39_34)).value(),
            u16::try_from(s_39_40).unwrap(),
        ));
        // D s_39_42: cast reint s_39_41 -> u8
        let s_39_42: bool = ((s_39_41.value()) != 0);
        // C s_39_43: const #16971u : u32
        let s_39_43: u32 = 16971;
        // N s_39_44: write-reg s_39_43 <= s_39_42
        let s_39_44: () = {
            state.write_register::<bool>(s_39_43 as isize, s_39_42);
            tracer.write_register(s_39_43 as isize, s_39_42);
        };
        // C s_39_45: const #0s : i
        let s_39_45: i128 = 0;
        // D s_39_46: read-var nzcv:u8
        let s_39_46: u8 = fn_state.nzcv;
        // D s_39_47: cast zx s_39_46 -> bv
        let s_39_47: Bits = Bits::new(s_39_46 as u128, 4u16);
        // C s_39_48: const #1s : i64
        let s_39_48: i64 = 1;
        // C s_39_49: cast zx s_39_48 -> i
        let s_39_49: i128 = (i128::try_from(s_39_48).unwrap());
        // C s_39_50: const #0s : i
        let s_39_50: i128 = 0;
        // C s_39_51: add s_39_50 s_39_49
        let s_39_51: i128 = (s_39_50 + s_39_49);
        // D s_39_52: bit-extract s_39_47 s_39_45 s_39_51
        let s_39_52: Bits = (Bits::new(
            ((s_39_47) >> (s_39_45)).value(),
            u16::try_from(s_39_51).unwrap(),
        ));
        // D s_39_53: cast reint s_39_52 -> u8
        let s_39_53: bool = ((s_39_52.value()) != 0);
        // C s_39_54: const #16996u : u32
        let s_39_54: u32 = 16996;
        // N s_39_55: write-reg s_39_54 <= s_39_53
        let s_39_55: () = {
            state.write_register::<bool>(s_39_54 as isize, s_39_53);
            tracer.write_register(s_39_54 as isize, s_39_53);
        };
        // N s_39_56: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_40_0: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var stagesetsize:u64
        let s_41_0: u64 = fn_state.stagesetsize;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 64u16);
        // D s_41_2: cast sx s_41_1 -> i
        let s_41_2: i128 = {
            let sign_bit = s_41_1.length() - 1;
            let mut result = s_41_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: const #0s : i
        let s_41_4: i128 = 0;
        // D s_41_5: cast zx s_41_3 -> i
        let s_41_5: i128 = (i128::try_from(s_41_3).unwrap());
        // D s_41_6: cmp-lt s_41_5 s_41_4
        let s_41_6: bool = ((s_41_5) < (s_41_4));
        // D s_41_7: not s_41_6
        let s_41_7: bool = !s_41_6;
        // N s_41_8: branch s_41_7 b51 b42
        if s_41_7 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #16s : i
        let s_42_0: i128 = 16;
        // D s_42_1: read-var toaddress:u64
        let s_42_1: u64 = fn_state.toaddress;
        // D s_42_2: read-var setsize:u64
        let s_42_2: u64 = fn_state.setsize;
        // D s_42_3: call SETSizeChoice(s_42_1, s_42_2, s_42_0)
        let s_42_3: i128 = SETSizeChoice(state, tracer, s_42_1, s_42_2, s_42_0);
        // D s_42_4: write-var B <= s_42_3
        fn_state.B = s_42_3;
        // C s_42_5: const #1s : i
        let s_42_5: i128 = 1;
        // C s_42_6: neg s_42_5
        let s_42_6: i128 = -s_42_5;
        // C s_42_7: cast reint s_42_6 -> i64
        let s_42_7: i64 = (s_42_6 as i64);
        // D s_42_8: read-var stagesetsize:u64
        let s_42_8: u64 = fn_state.stagesetsize;
        // D s_42_9: cast zx s_42_8 -> bv
        let s_42_9: Bits = Bits::new(s_42_8 as u128, 64u16);
        // D s_42_10: cast sx s_42_9 -> i
        let s_42_10: i128 = {
            let sign_bit = s_42_9.length() - 1;
            let mut result = s_42_9.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_42_11: cast reint s_42_10 -> i64
        let s_42_11: i64 = (s_42_10 as i64);
        // C s_42_12: cast zx s_42_7 -> i
        let s_42_12: i128 = (i128::try_from(s_42_7).unwrap());
        // D s_42_13: cast zx s_42_11 -> i
        let s_42_13: i128 = (i128::try_from(s_42_11).unwrap());
        // D s_42_14: mul s_42_12 s_42_13
        let s_42_14: i128 = ((s_42_12) * (s_42_13));
        // D s_42_15: read-var B:i
        let s_42_15: i128 = fn_state.B;
        // D s_42_16: cmp-le s_42_15 s_42_14
        let s_42_16: bool = ((s_42_15) <= (s_42_14));
        // N s_42_17: assert s_42_16
        let s_42_17: () = assert!(s_42_16);
        // C s_42_18: const #3s : i
        let s_42_18: i128 = 3;
        // C s_42_19: const #0s : i
        let s_42_19: i128 = 0;
        // D s_42_20: read-var B:i
        let s_42_20: i128 = fn_state.B;
        // D s_42_21: call integer_subrange(s_42_20, s_42_18, s_42_19)
        let s_42_21: Bits = integer_subrange(state, tracer, s_42_20, s_42_18, s_42_19);
        // D s_42_22: cast reint s_42_21 -> u8
        let s_42_22: u8 = (s_42_21.value() as u8);
        // D s_42_23: cast zx s_42_22 -> bv
        let s_42_23: Bits = Bits::new(s_42_22 as u128, 4u16);
        // C s_42_24: const #0u : u8
        let s_42_24: u8 = 0;
        // C s_42_25: cast zx s_42_24 -> bv
        let s_42_25: Bits = Bits::new(s_42_24 as u128, 4u16);
        // D s_42_26: cmp-eq s_42_23 s_42_25
        let s_42_26: bool = ((s_42_23) == (s_42_25));
        // N s_42_27: assert s_42_26
        let s_42_27: () = assert!(s_42_26);
        // D s_42_28: read-var B:i
        let s_42_28: i128 = fn_state.B;
        // D s_42_29: call __id(s_42_28)
        let s_42_29: i128 = u__id(state, tracer, s_42_28);
        // C s_42_30: const #8s : i
        let s_42_30: i128 = 8;
        // D s_42_31: mul s_42_29 s_42_30
        let s_42_31: i128 = ((s_42_29) * (s_42_30));
        // C s_42_32: const #0s : i
        let s_42_32: i128 = 0;
        // D s_42_33: cmp-ge s_42_31 s_42_32
        let s_42_33: bool = ((s_42_31) >= (s_42_32));
        // N s_42_34: assert s_42_33
        let s_42_34: () = assert!(s_42_33);
        // D s_42_35: read-var toaddress:u64
        let s_42_35: u64 = fn_state.toaddress;
        // D s_42_36: cast zx s_42_35 -> bv
        let s_42_36: Bits = Bits::new(s_42_35 as u128, 64u16);
        // D s_42_37: read-var setsize:u64
        let s_42_37: u64 = fn_state.setsize;
        // D s_42_38: cast zx s_42_37 -> bv
        let s_42_38: Bits = Bits::new(s_42_37 as u128, 64u16);
        // D s_42_39: add s_42_36 s_42_38
        let s_42_39: Bits = (s_42_36 + s_42_38);
        // D s_42_40: cast reint s_42_39 -> u64
        let s_42_40: u64 = (s_42_39.value() as u64);
        // D s_42_41: read-var data:u8
        let s_42_41: u8 = fn_state.data;
        // D s_42_42: cast zx s_42_41 -> bv
        let s_42_42: Bits = Bits::new(s_42_41 as u128, 8u16);
        // D s_42_43: cast reint s_42_28 -> u64
        let s_42_43: u64 = (s_42_28 as u64);
        // D s_42_44: call replicate_bits_borealis_internal(s_42_42, s_42_43)
        let s_42_44: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_42_42,
            s_42_43,
        );
        // D s_42_45: read-var accdesc:struct
        let s_42_45: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_42_46: call Mem_set(s_42_40, s_42_28, s_42_45, s_42_44)
        let s_42_46: () = Mem_set(state, tracer, s_42_40, s_42_28, s_42_45, s_42_44);
        // N s_42_47: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #16s : i
        let s_43_0: i128 = 16;
        // D s_43_1: read-var B:i
        let s_43_1: i128 = fn_state.B;
        // D s_43_2: call fdiv_int(s_43_1, s_43_0)
        let s_43_2: i128 = fdiv_int(state, tracer, s_43_1, s_43_0);
        // D s_43_3: write-var tagstep <= s_43_2
        fn_state.tagstep = s_43_2;
        // D s_43_4: read-var toaddress:u64
        let s_43_4: u64 = fn_state.toaddress;
        // D s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 64u16);
        // D s_43_6: read-var setsize:u64
        let s_43_6: u64 = fn_state.setsize;
        // D s_43_7: cast zx s_43_6 -> bv
        let s_43_7: Bits = Bits::new(s_43_6 as u128, 64u16);
        // D s_43_8: add s_43_5 s_43_7
        let s_43_8: Bits = (s_43_5 + s_43_7);
        // D s_43_9: cast reint s_43_8 -> u64
        let s_43_9: u64 = (s_43_8.value() as u64);
        // D s_43_10: call AArch64_AllocationTagFromAddress(s_43_9)
        let s_43_10: u8 = AArch64_AllocationTagFromAddress(state, tracer, s_43_9);
        // D s_43_11: write-var tag <= s_43_10
        fn_state.tag = s_43_10;
        // N s_43_12: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0s : i
        let s_44_0: i128 = 0;
        // D s_44_1: read-var tagstep:i
        let s_44_1: i128 = fn_state.tagstep;
        // D s_44_2: cmp-gt s_44_1 s_44_0
        let s_44_2: bool = ((s_44_1) > (s_44_0));
        // D s_44_3: not s_44_2
        let s_44_3: bool = !s_44_2;
        // N s_44_4: branch s_44_3 b47 b45
        if s_44_3 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var toaddress:u64
        let s_45_0: u64 = fn_state.toaddress;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 64u16);
        // D s_45_2: read-var setsize:u64
        let s_45_2: u64 = fn_state.setsize;
        // D s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 64u16);
        // D s_45_4: add s_45_1 s_45_3
        let s_45_4: Bits = (s_45_1 + s_45_3);
        // D s_45_5: cast reint s_45_4 -> u64
        let s_45_5: u64 = (s_45_4.value() as u64);
        // C s_45_6: const #1s : i
        let s_45_6: i128 = 1;
        // D s_45_7: read-var tagstep:i
        let s_45_7: i128 = fn_state.tagstep;
        // D s_45_8: sub s_45_7 s_45_6
        let s_45_8: i128 = ((s_45_7) - (s_45_6));
        // C s_45_9: const #16s : i
        let s_45_9: i128 = 16;
        // D s_45_10: mul s_45_8 s_45_9
        let s_45_10: i128 = ((s_45_8) * (s_45_9));
        // D s_45_11: cast zx s_45_5 -> bv
        let s_45_11: Bits = Bits::new(s_45_5 as u128, 64u16);
        // D s_45_12: cast cvt s_45_10 -> bv
        let s_45_12: Bits = Bits::new(s_45_10 as u128, 128);
        // D s_45_13: add s_45_11 s_45_12
        let s_45_13: Bits = (s_45_11 + s_45_12);
        // D s_45_14: cast reint s_45_13 -> u64
        let s_45_14: u64 = (s_45_13.value() as u64);
        // D s_45_15: write-var tagaddr <= s_45_14
        fn_state.tagaddr = s_45_14;
        // D s_45_16: read-var tagaddr:u64
        let s_45_16: u64 = fn_state.tagaddr;
        // D s_45_17: read-var accdesc:struct
        let s_45_17: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_45_18: read-var tag:u8
        let s_45_18: u8 = fn_state.tag;
        // D s_45_19: call AArch64_MemTag_set(s_45_16, s_45_17, s_45_18)
        let s_45_19: () = AArch64_MemTag_set(state, tracer, s_45_16, s_45_17, s_45_18);
        // N s_45_20: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1s : i
        let s_46_0: i128 = 1;
        // D s_46_1: read-var tagstep:i
        let s_46_1: i128 = fn_state.tagstep;
        // D s_46_2: sub s_46_1 s_46_0
        let s_46_2: i128 = ((s_46_1) - (s_46_0));
        // D s_46_3: write-var tagstep <= s_46_2
        fn_state.tagstep = s_46_2;
        // N s_46_4: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var setsize:u64
        let s_47_0: u64 = fn_state.setsize;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 64u16);
        // D s_47_2: read-var B:i
        let s_47_2: i128 = fn_state.B;
        // D s_47_3: cast cvt s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 128);
        // D s_47_4: add s_47_1 s_47_3
        let s_47_4: Bits = (s_47_1 + s_47_3);
        // D s_47_5: cast reint s_47_4 -> u64
        let s_47_5: u64 = (s_47_4.value() as u64);
        // D s_47_6: write-var setsize <= s_47_5
        fn_state.setsize = s_47_5;
        // D s_47_7: read-var stagesetsize:u64
        let s_47_7: u64 = fn_state.stagesetsize;
        // D s_47_8: cast zx s_47_7 -> bv
        let s_47_8: Bits = Bits::new(s_47_7 as u128, 64u16);
        // D s_47_9: read-var B:i
        let s_47_9: i128 = fn_state.B;
        // D s_47_10: cast cvt s_47_9 -> bv
        let s_47_10: Bits = Bits::new(s_47_9 as u128, 128);
        // D s_47_11: add s_47_8 s_47_10
        let s_47_11: Bits = (s_47_8 + s_47_10);
        // D s_47_12: cast reint s_47_11 -> u64
        let s_47_12: u64 = (s_47_11.value() as u64);
        // D s_47_13: write-var stagesetsize <= s_47_12
        fn_state.stagesetsize = s_47_12;
        // D s_47_14: read-var stage:u32
        let s_47_14: u32 = fn_state.stage;
        // C s_47_15: const #0u : u32
        let s_47_15: u32 = 0;
        // D s_47_16: cmp-eq s_47_14 s_47_15
        let s_47_16: bool = ((s_47_14) == (s_47_15));
        // N s_47_17: branch s_47_16 b50 b48
        if s_47_16 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_49_0: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #64s : i64
        let s_50_0: i64 = 64;
        // D s_50_1: read-var n:i64
        let s_50_1: i64 = fn_state.n;
        // D s_50_2: cast zx s_50_1 -> i
        let s_50_2: i128 = (i128::try_from(s_50_1).unwrap());
        // D s_50_3: read-var setsize:u64
        let s_50_3: u64 = fn_state.setsize;
        // D s_50_4: cast zx s_50_3 -> bv
        let s_50_4: Bits = Bits::new(s_50_3 as u128, 64u16);
        // D s_50_5: call X_set(s_50_2, s_50_0, s_50_4)
        let s_50_5: () = X_set(state, tracer, s_50_2, s_50_0, s_50_4);
        // N s_50_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_51_0: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call SPESampleMemSet(s_52_0)
        let s_52_1: () = SPESampleMemSet(state, tracer, s_52_0);
        // N s_52_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var accdesc:struct
        let s_53_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_53_1: call AlignmentFault(s_53_0)
        let s_53_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_53_0);
        // D s_53_2: read-var toaddress:u64
        let s_53_2: u64 = fn_state.toaddress;
        // D s_53_3: call AArch64_Abort(s_53_2, s_53_1)
        let s_53_3: () = AArch64_Abort(state, tracer, s_53_2, s_53_1);
        // N s_53_4: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var accdesc:struct
        let s_54_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_54_1: call AlignmentFault(s_54_0)
        let s_54_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_54_0);
        // D s_54_2: read-var toaddress:u64
        let s_54_2: u64 = fn_state.toaddress;
        // D s_54_3: call AArch64_Abort(s_54_2, s_54_1)
        let s_54_3: () = AArch64_Abort(state, tracer, s_54_2, s_54_1);
        // N s_54_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var toaddress:u64
        let s_55_0: u64 = fn_state.toaddress;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 64u16);
        // C s_55_2: const #1344u : u32
        let s_55_2: u32 = 1344;
        // D s_55_3: read-reg s_55_2:i64
        let s_55_3: i64 = {
            let value = state.read_register::<i64>(s_55_2 as isize);
            tracer.read_register(s_55_2 as isize, value);
            value
        };
        // D s_55_4: cast zx s_55_3 -> i
        let s_55_4: i128 = (i128::try_from(s_55_3).unwrap());
        // D s_55_5: call IsAligned__1(s_55_1, s_55_4)
        let s_55_5: bool = IsAligned__1(state, tracer, s_55_1, s_55_4);
        // D s_55_6: not s_55_5
        let s_55_6: bool = !s_55_5;
        // D s_55_7: write-var gs#168062 <= s_55_6
        fn_state.gs_168062 = s_55_6;
        // N s_55_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // C s_56_1: const #1u : u8
        let s_56_1: bool = true;
        // D s_56_2: read-var d:i64
        let s_56_2: i64 = fn_state.d;
        // D s_56_3: cast zx s_56_2 -> i
        let s_56_3: i128 = (i128::try_from(s_56_2).unwrap());
        // D s_56_4: read-var s:i64
        let s_56_4: i64 = fn_state.s;
        // D s_56_5: cast zx s_56_4 -> i
        let s_56_5: i128 = (i128::try_from(s_56_4).unwrap());
        // D s_56_6: read-var n:i64
        let s_56_6: i64 = fn_state.n;
        // D s_56_7: cast zx s_56_6 -> i
        let s_56_7: i128 = (i128::try_from(s_56_6).unwrap());
        // D s_56_8: read-var supports_option_a:u8
        let s_56_8: bool = fn_state.supports_option_a;
        // D s_56_9: read-var options_name:u8
        let s_56_9: u8 = fn_state.options_name;
        // C s_56_10: const #1u : u8
        let s_56_10: bool = true;
        // D s_56_11: call MismatchedMemSetException(s_56_8, s_56_3, s_56_5, s_56_7, s_56_0, s_56_1, s_56_9, s_56_10)
        let s_56_11: () = MismatchedMemSetException(
            state,
            tracer,
            s_56_8,
            s_56_3,
            s_56_5,
            s_56_7,
            s_56_0,
            s_56_1,
            s_56_9,
            s_56_10,
        );
        // N s_56_12: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#168066 <= s_57_0
        fn_state.gs_168066 = s_57_0;
        // N s_57_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var setsize:u64
        let s_58_0: u64 = fn_state.setsize;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 64u16);
        // D s_58_2: read-var postsize:u64
        let s_58_2: u64 = fn_state.postsize;
        // D s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 64u16);
        // D s_58_4: sub s_58_1 s_58_3
        let s_58_4: Bits = ((s_58_1) - (s_58_3));
        // D s_58_5: cast reint s_58_4 -> u64
        let s_58_5: u64 = (s_58_4.value() as u64);
        // D s_58_6: write-var stagesetsize <= s_58_5
        fn_state.stagesetsize = s_58_5;
        // D s_58_7: read-var toaddress:u64
        let s_58_7: u64 = fn_state.toaddress;
        // D s_58_8: read-var setsize:u64
        let s_58_8: u64 = fn_state.setsize;
        // C s_58_9: const #1u : u8
        let s_58_9: bool = true;
        // D s_58_10: call MemSetParametersIllformedM(s_58_7, s_58_8, s_58_9)
        let s_58_10: bool = MemSetParametersIllformedM(
            state,
            tracer,
            s_58_7,
            s_58_8,
            s_58_9,
        );
        // N s_58_11: branch s_58_10 b61 b59
        if s_58_10 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_60_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // C s_61_1: const #0u : u8
        let s_61_1: bool = false;
        // D s_61_2: read-var d:i64
        let s_61_2: i64 = fn_state.d;
        // D s_61_3: cast zx s_61_2 -> i
        let s_61_3: i128 = (i128::try_from(s_61_2).unwrap());
        // D s_61_4: read-var s:i64
        let s_61_4: i64 = fn_state.s;
        // D s_61_5: cast zx s_61_4 -> i
        let s_61_5: i128 = (i128::try_from(s_61_4).unwrap());
        // D s_61_6: read-var n:i64
        let s_61_6: i64 = fn_state.n;
        // D s_61_7: cast zx s_61_6 -> i
        let s_61_7: i128 = (i128::try_from(s_61_6).unwrap());
        // D s_61_8: read-var supports_option_a:u8
        let s_61_8: bool = fn_state.supports_option_a;
        // D s_61_9: read-var options_name:u8
        let s_61_9: u8 = fn_state.options_name;
        // C s_61_10: const #1u : u8
        let s_61_10: bool = true;
        // D s_61_11: call MismatchedMemSetException(s_61_8, s_61_3, s_61_5, s_61_7, s_61_0, s_61_1, s_61_9, s_61_10)
        let s_61_11: () = MismatchedMemSetException(
            state,
            tracer,
            s_61_8,
            s_61_3,
            s_61_5,
            s_61_7,
            s_61_0,
            s_61_1,
            s_61_9,
            s_61_10,
        );
        // N s_61_12: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var supports_option_a:u8
        let s_62_0: bool = fn_state.supports_option_a;
        // N s_62_1: branch s_62_0 b67 b63
        if s_62_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1s : i
        let s_63_0: i128 = 1;
        // D s_63_1: read-var nzcv:u8
        let s_63_1: u8 = fn_state.nzcv;
        // D s_63_2: cast zx s_63_1 -> bv
        let s_63_2: Bits = Bits::new(s_63_1 as u128, 4u16);
        // C s_63_3: const #1u : u64
        let s_63_3: u64 = 1;
        // D s_63_4: bit-extract s_63_2 s_63_0 s_63_3
        let s_63_4: Bits = (Bits::new(
            ((s_63_2) >> (s_63_0)).value(),
            u16::try_from(s_63_3).unwrap(),
        ));
        // D s_63_5: cast reint s_63_4 -> u8
        let s_63_5: bool = ((s_63_4.value()) != 0);
        // C s_63_6: const #0s : i
        let s_63_6: i128 = 0;
        // C s_63_7: const #0u : u64
        let s_63_7: u64 = 0;
        // D s_63_8: cast zx s_63_5 -> u64
        let s_63_8: u64 = (s_63_5 as u64);
        // C s_63_9: const #1u : u64
        let s_63_9: u64 = 1;
        // D s_63_10: and s_63_8 s_63_9
        let s_63_10: u64 = ((s_63_8) & (s_63_9));
        // D s_63_11: cmp-eq s_63_10 s_63_9
        let s_63_11: bool = ((s_63_10) == (s_63_9));
        // D s_63_12: lsl s_63_8 s_63_6
        let s_63_12: u64 = s_63_8 << s_63_6;
        // D s_63_13: or s_63_7 s_63_12
        let s_63_13: u64 = ((s_63_7) | (s_63_12));
        // D s_63_14: cmpl s_63_12
        let s_63_14: u64 = !s_63_12;
        // D s_63_15: and s_63_7 s_63_14
        let s_63_15: u64 = ((s_63_7) & (s_63_14));
        // D s_63_16: select s_63_11 s_63_13 s_63_15
        let s_63_16: u64 = if s_63_11 { s_63_13 } else { s_63_15 };
        // D s_63_17: cast trunc s_63_16 -> u8
        let s_63_17: bool = ((s_63_16) != 0);
        // D s_63_18: cast zx s_63_17 -> bv
        let s_63_18: Bits = Bits::new(s_63_17 as u128, 1u16);
        // C s_63_19: const #0u : u8
        let s_63_19: bool = false;
        // C s_63_20: cast zx s_63_19 -> bv
        let s_63_20: Bits = Bits::new(s_63_19 as u128, 1u16);
        // D s_63_21: cmp-eq s_63_18 s_63_20
        let s_63_21: bool = ((s_63_18) == (s_63_20));
        // N s_63_22: branch s_63_21 b66 b64
        if s_63_21 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_64_0: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_65_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: read-var stage:u32
        let s_66_1: u32 = fn_state.stage;
        // C s_66_2: const #2u : u32
        let s_66_2: u32 = 2;
        // D s_66_3: cmp-eq s_66_1 s_66_2
        let s_66_3: bool = ((s_66_1) == (s_66_2));
        // D s_66_4: read-var d:i64
        let s_66_4: i64 = fn_state.d;
        // D s_66_5: cast zx s_66_4 -> i
        let s_66_5: i128 = (i128::try_from(s_66_4).unwrap());
        // D s_66_6: read-var s:i64
        let s_66_6: i64 = fn_state.s;
        // D s_66_7: cast zx s_66_6 -> i
        let s_66_7: i128 = (i128::try_from(s_66_6).unwrap());
        // D s_66_8: read-var n:i64
        let s_66_8: i64 = fn_state.n;
        // D s_66_9: cast zx s_66_8 -> i
        let s_66_9: i128 = (i128::try_from(s_66_8).unwrap());
        // D s_66_10: read-var supports_option_a:u8
        let s_66_10: bool = fn_state.supports_option_a;
        // D s_66_11: read-var options_name:u8
        let s_66_11: u8 = fn_state.options_name;
        // C s_66_12: const #1u : u8
        let s_66_12: bool = true;
        // D s_66_13: call MismatchedMemSetException(s_66_10, s_66_5, s_66_7, s_66_9, s_66_0, s_66_3, s_66_11, s_66_12)
        let s_66_13: () = MismatchedMemSetException(
            state,
            tracer,
            s_66_10,
            s_66_5,
            s_66_7,
            s_66_9,
            s_66_0,
            s_66_3,
            s_66_11,
            s_66_12,
        );
        // N s_66_14: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1s : i
        let s_67_0: i128 = 1;
        // D s_67_1: read-var nzcv:u8
        let s_67_1: u8 = fn_state.nzcv;
        // D s_67_2: cast zx s_67_1 -> bv
        let s_67_2: Bits = Bits::new(s_67_1 as u128, 4u16);
        // C s_67_3: const #1u : u64
        let s_67_3: u64 = 1;
        // D s_67_4: bit-extract s_67_2 s_67_0 s_67_3
        let s_67_4: Bits = (Bits::new(
            ((s_67_2) >> (s_67_0)).value(),
            u16::try_from(s_67_3).unwrap(),
        ));
        // D s_67_5: cast reint s_67_4 -> u8
        let s_67_5: bool = ((s_67_4.value()) != 0);
        // C s_67_6: const #0s : i
        let s_67_6: i128 = 0;
        // C s_67_7: const #0u : u64
        let s_67_7: u64 = 0;
        // D s_67_8: cast zx s_67_5 -> u64
        let s_67_8: u64 = (s_67_5 as u64);
        // C s_67_9: const #1u : u64
        let s_67_9: u64 = 1;
        // D s_67_10: and s_67_8 s_67_9
        let s_67_10: u64 = ((s_67_8) & (s_67_9));
        // D s_67_11: cmp-eq s_67_10 s_67_9
        let s_67_11: bool = ((s_67_10) == (s_67_9));
        // D s_67_12: lsl s_67_8 s_67_6
        let s_67_12: u64 = s_67_8 << s_67_6;
        // D s_67_13: or s_67_7 s_67_12
        let s_67_13: u64 = ((s_67_7) | (s_67_12));
        // D s_67_14: cmpl s_67_12
        let s_67_14: u64 = !s_67_12;
        // D s_67_15: and s_67_7 s_67_14
        let s_67_15: u64 = ((s_67_7) & (s_67_14));
        // D s_67_16: select s_67_11 s_67_13 s_67_15
        let s_67_16: u64 = if s_67_11 { s_67_13 } else { s_67_15 };
        // D s_67_17: cast trunc s_67_16 -> u8
        let s_67_17: bool = ((s_67_16) != 0);
        // D s_67_18: cast zx s_67_17 -> bv
        let s_67_18: Bits = Bits::new(s_67_17 as u128, 1u16);
        // C s_67_19: const #1u : u8
        let s_67_19: bool = true;
        // C s_67_20: cast zx s_67_19 -> bv
        let s_67_20: Bits = Bits::new(s_67_19 as u128, 1u16);
        // D s_67_21: cmp-eq s_67_18 s_67_20
        let s_67_21: bool = ((s_67_18) == (s_67_20));
        // N s_67_22: branch s_67_21 b70 b68
        if s_67_21 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_68_0: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_69_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: read-var stage:u32
        let s_70_1: u32 = fn_state.stage;
        // C s_70_2: const #2u : u32
        let s_70_2: u32 = 2;
        // D s_70_3: cmp-eq s_70_1 s_70_2
        let s_70_3: bool = ((s_70_1) == (s_70_2));
        // D s_70_4: read-var d:i64
        let s_70_4: i64 = fn_state.d;
        // D s_70_5: cast zx s_70_4 -> i
        let s_70_5: i128 = (i128::try_from(s_70_4).unwrap());
        // D s_70_6: read-var s:i64
        let s_70_6: i64 = fn_state.s;
        // D s_70_7: cast zx s_70_6 -> i
        let s_70_7: i128 = (i128::try_from(s_70_6).unwrap());
        // D s_70_8: read-var n:i64
        let s_70_8: i64 = fn_state.n;
        // D s_70_9: cast zx s_70_8 -> i
        let s_70_9: i128 = (i128::try_from(s_70_8).unwrap());
        // D s_70_10: read-var supports_option_a:u8
        let s_70_10: bool = fn_state.supports_option_a;
        // D s_70_11: read-var options_name:u8
        let s_70_11: u8 = fn_state.options_name;
        // C s_70_12: const #1u : u8
        let s_70_12: bool = true;
        // D s_70_13: call MismatchedMemSetException(s_70_10, s_70_5, s_70_7, s_70_9, s_70_0, s_70_3, s_70_11, s_70_12)
        let s_70_13: () = MismatchedMemSetException(
            state,
            tracer,
            s_70_10,
            s_70_5,
            s_70_7,
            s_70_9,
            s_70_0,
            s_70_3,
            s_70_11,
            s_70_12,
        );
        // N s_70_14: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#168060 <= s_71_0
        fn_state.gs_168060 = s_71_0;
        // N s_71_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#168054 <= s_72_0
        fn_state.gs_168054 = s_72_0;
        // N s_72_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #63s : i
        let s_73_0: i128 = 63;
        // D s_73_1: read-var setsize:u64
        let s_73_1: u64 = fn_state.setsize;
        // D s_73_2: cast zx s_73_1 -> bv
        let s_73_2: Bits = Bits::new(s_73_1 as u128, 64u16);
        // C s_73_3: const #1u : u64
        let s_73_3: u64 = 1;
        // D s_73_4: bit-extract s_73_2 s_73_0 s_73_3
        let s_73_4: Bits = (Bits::new(
            ((s_73_2) >> (s_73_0)).value(),
            u16::try_from(s_73_3).unwrap(),
        ));
        // D s_73_5: cast reint s_73_4 -> u8
        let s_73_5: bool = ((s_73_4.value()) != 0);
        // C s_73_6: const #0s : i
        let s_73_6: i128 = 0;
        // C s_73_7: const #0u : u64
        let s_73_7: u64 = 0;
        // D s_73_8: cast zx s_73_5 -> u64
        let s_73_8: u64 = (s_73_5 as u64);
        // C s_73_9: const #1u : u64
        let s_73_9: u64 = 1;
        // D s_73_10: and s_73_8 s_73_9
        let s_73_10: u64 = ((s_73_8) & (s_73_9));
        // D s_73_11: cmp-eq s_73_10 s_73_9
        let s_73_11: bool = ((s_73_10) == (s_73_9));
        // D s_73_12: lsl s_73_8 s_73_6
        let s_73_12: u64 = s_73_8 << s_73_6;
        // D s_73_13: or s_73_7 s_73_12
        let s_73_13: u64 = ((s_73_7) | (s_73_12));
        // D s_73_14: cmpl s_73_12
        let s_73_14: u64 = !s_73_12;
        // D s_73_15: and s_73_7 s_73_14
        let s_73_15: u64 = ((s_73_7) & (s_73_14));
        // D s_73_16: select s_73_11 s_73_13 s_73_15
        let s_73_16: u64 = if s_73_11 { s_73_13 } else { s_73_15 };
        // D s_73_17: cast trunc s_73_16 -> u8
        let s_73_17: bool = ((s_73_16) != 0);
        // D s_73_18: cast zx s_73_17 -> bv
        let s_73_18: Bits = Bits::new(s_73_17 as u128, 1u16);
        // C s_73_19: const #1u : u8
        let s_73_19: bool = true;
        // C s_73_20: cast zx s_73_19 -> bv
        let s_73_20: Bits = Bits::new(s_73_19 as u128, 1u16);
        // D s_73_21: cmp-eq s_73_18 s_73_20
        let s_73_21: bool = ((s_73_18) == (s_73_20));
        // N s_73_22: branch s_73_21 b93 b74
        if s_73_21 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_74_0: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #64s : i
        let s_75_0: i128 = 64;
        // S s_75_1: call Zeros(s_75_0)
        let s_75_1: Bits = Zeros(state, tracer, s_75_0);
        // S s_75_2: cast reint s_75_1 -> u64
        let s_75_2: u64 = (s_75_1.value() as u64);
        // D s_75_3: read-var setsize:u64
        let s_75_3: u64 = fn_state.setsize;
        // D s_75_4: cast zx s_75_3 -> bv
        let s_75_4: Bits = Bits::new(s_75_3 as u128, 64u16);
        // S s_75_5: cast zx s_75_2 -> bv
        let s_75_5: Bits = Bits::new(s_75_2 as u128, 64u16);
        // D s_75_6: cmp-ne s_75_4 s_75_5
        let s_75_6: bool = ((s_75_4) != (s_75_5));
        // N s_75_7: branch s_75_6 b92 b76
        if s_75_6 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#168081 <= s_76_0
        fn_state.gs_168081 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#168081:u8
        let s_77_0: bool = fn_state.gs_168081;
        // N s_77_1: branch s_77_0 b91 b78
        if s_77_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_78_0: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var setsize:u64
        let s_79_0: u64 = fn_state.setsize;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 64u16);
        // C s_79_2: const #1344u : u32
        let s_79_2: u32 = 1344;
        // D s_79_3: read-reg s_79_2:i64
        let s_79_3: i64 = {
            let value = state.read_register::<i64>(s_79_2 as isize);
            tracer.read_register(s_79_2 as isize, value);
            value
        };
        // D s_79_4: cast zx s_79_3 -> i
        let s_79_4: i128 = (i128::try_from(s_79_3).unwrap());
        // D s_79_5: call IsAligned__1(s_79_1, s_79_4)
        let s_79_5: bool = IsAligned__1(state, tracer, s_79_1, s_79_4);
        // D s_79_6: not s_79_5
        let s_79_6: bool = !s_79_5;
        // N s_79_7: branch s_79_6 b90 b80
        if s_79_6 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_80_0: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var supports_option_a:u8
        let s_81_0: bool = fn_state.supports_option_a;
        // N s_81_1: branch s_81_0 b89 b82
        if s_81_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #2u : u8
        let s_82_0: u8 = 2;
        // D s_82_1: write-var nzcv <= s_82_0
        fn_state.nzcv = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var toaddress:u64
        let s_83_0: u64 = fn_state.toaddress;
        // D s_83_1: read-var setsize:u64
        let s_83_1: u64 = fn_state.setsize;
        // C s_83_2: const #1u : u8
        let s_83_2: bool = true;
        // D s_83_3: call SETPreSizeChoice(s_83_0, s_83_1, s_83_2)
        let s_83_3: u64 = SETPreSizeChoice(state, tracer, s_83_0, s_83_1, s_83_2);
        // D s_83_4: write-var stagesetsize <= s_83_3
        fn_state.stagesetsize = s_83_3;
        // C s_83_5: const #63s : i
        let s_83_5: i128 = 63;
        // D s_83_6: read-var stagesetsize:u64
        let s_83_6: u64 = fn_state.stagesetsize;
        // D s_83_7: cast zx s_83_6 -> bv
        let s_83_7: Bits = Bits::new(s_83_6 as u128, 64u16);
        // C s_83_8: const #1u : u64
        let s_83_8: u64 = 1;
        // D s_83_9: bit-extract s_83_7 s_83_5 s_83_8
        let s_83_9: Bits = (Bits::new(
            ((s_83_7) >> (s_83_5)).value(),
            u16::try_from(s_83_8).unwrap(),
        ));
        // D s_83_10: cast reint s_83_9 -> u8
        let s_83_10: bool = ((s_83_9.value()) != 0);
        // C s_83_11: const #0s : i
        let s_83_11: i128 = 0;
        // C s_83_12: const #0u : u64
        let s_83_12: u64 = 0;
        // D s_83_13: cast zx s_83_10 -> u64
        let s_83_13: u64 = (s_83_10 as u64);
        // C s_83_14: const #1u : u64
        let s_83_14: u64 = 1;
        // D s_83_15: and s_83_13 s_83_14
        let s_83_15: u64 = ((s_83_13) & (s_83_14));
        // D s_83_16: cmp-eq s_83_15 s_83_14
        let s_83_16: bool = ((s_83_15) == (s_83_14));
        // D s_83_17: lsl s_83_13 s_83_11
        let s_83_17: u64 = s_83_13 << s_83_11;
        // D s_83_18: or s_83_12 s_83_17
        let s_83_18: u64 = ((s_83_12) | (s_83_17));
        // D s_83_19: cmpl s_83_17
        let s_83_19: u64 = !s_83_17;
        // D s_83_20: and s_83_12 s_83_19
        let s_83_20: u64 = ((s_83_12) & (s_83_19));
        // D s_83_21: select s_83_16 s_83_18 s_83_20
        let s_83_21: u64 = if s_83_16 { s_83_18 } else { s_83_20 };
        // D s_83_22: cast trunc s_83_21 -> u8
        let s_83_22: bool = ((s_83_21) != 0);
        // C s_83_23: const #63s : i
        let s_83_23: i128 = 63;
        // D s_83_24: read-var setsize:u64
        let s_83_24: u64 = fn_state.setsize;
        // D s_83_25: cast zx s_83_24 -> bv
        let s_83_25: Bits = Bits::new(s_83_24 as u128, 64u16);
        // C s_83_26: const #1u : u64
        let s_83_26: u64 = 1;
        // D s_83_27: bit-extract s_83_25 s_83_23 s_83_26
        let s_83_27: Bits = (Bits::new(
            ((s_83_25) >> (s_83_23)).value(),
            u16::try_from(s_83_26).unwrap(),
        ));
        // D s_83_28: cast reint s_83_27 -> u8
        let s_83_28: bool = ((s_83_27.value()) != 0);
        // C s_83_29: const #0s : i
        let s_83_29: i128 = 0;
        // C s_83_30: const #0u : u64
        let s_83_30: u64 = 0;
        // D s_83_31: cast zx s_83_28 -> u64
        let s_83_31: u64 = (s_83_28 as u64);
        // C s_83_32: const #1u : u64
        let s_83_32: u64 = 1;
        // D s_83_33: and s_83_31 s_83_32
        let s_83_33: u64 = ((s_83_31) & (s_83_32));
        // D s_83_34: cmp-eq s_83_33 s_83_32
        let s_83_34: bool = ((s_83_33) == (s_83_32));
        // D s_83_35: lsl s_83_31 s_83_29
        let s_83_35: u64 = s_83_31 << s_83_29;
        // D s_83_36: or s_83_30 s_83_35
        let s_83_36: u64 = ((s_83_30) | (s_83_35));
        // D s_83_37: cmpl s_83_35
        let s_83_37: u64 = !s_83_35;
        // D s_83_38: and s_83_30 s_83_37
        let s_83_38: u64 = ((s_83_30) & (s_83_37));
        // D s_83_39: select s_83_34 s_83_36 s_83_38
        let s_83_39: u64 = if s_83_34 { s_83_36 } else { s_83_38 };
        // D s_83_40: cast trunc s_83_39 -> u8
        let s_83_40: bool = ((s_83_39) != 0);
        // D s_83_41: cast zx s_83_22 -> bv
        let s_83_41: Bits = Bits::new(s_83_22 as u128, 1u16);
        // D s_83_42: cast zx s_83_40 -> bv
        let s_83_42: Bits = Bits::new(s_83_40 as u128, 1u16);
        // D s_83_43: cmp-eq s_83_41 s_83_42
        let s_83_43: bool = ((s_83_41) == (s_83_42));
        // N s_83_44: branch s_83_43 b88 b84
        if s_83_43 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #64s : i
        let s_84_0: i128 = 64;
        // S s_84_1: call Zeros(s_84_0)
        let s_84_1: Bits = Zeros(state, tracer, s_84_0);
        // S s_84_2: cast reint s_84_1 -> u64
        let s_84_2: u64 = (s_84_1.value() as u64);
        // D s_84_3: read-var stagesetsize:u64
        let s_84_3: u64 = fn_state.stagesetsize;
        // D s_84_4: cast zx s_84_3 -> bv
        let s_84_4: Bits = Bits::new(s_84_3 as u128, 64u16);
        // S s_84_5: cast zx s_84_2 -> bv
        let s_84_5: Bits = Bits::new(s_84_2 as u128, 64u16);
        // D s_84_6: cmp-eq s_84_4 s_84_5
        let s_84_6: bool = ((s_84_4) == (s_84_5));
        // D s_84_7: write-var gs#168087 <= s_84_6
        fn_state.gs_168087 = s_84_6;
        // N s_84_8: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#168087:u8
        let s_85_0: bool = fn_state.gs_168087;
        // N s_85_1: assert s_85_0
        let s_85_1: () = assert!(s_85_0);
        // C s_85_2: const #0s : i
        let s_85_2: i128 = 0;
        // D s_85_3: read-var stagesetsize:u64
        let s_85_3: u64 = fn_state.stagesetsize;
        // D s_85_4: cast zx s_85_3 -> bv
        let s_85_4: Bits = Bits::new(s_85_3 as u128, 64u16);
        // C s_85_5: const #1s : i64
        let s_85_5: i64 = 1;
        // C s_85_6: cast zx s_85_5 -> i
        let s_85_6: i128 = (i128::try_from(s_85_5).unwrap());
        // C s_85_7: const #3s : i
        let s_85_7: i128 = 3;
        // C s_85_8: add s_85_7 s_85_6
        let s_85_8: i128 = (s_85_7 + s_85_6);
        // D s_85_9: bit-extract s_85_4 s_85_2 s_85_8
        let s_85_9: Bits = (Bits::new(
            ((s_85_4) >> (s_85_2)).value(),
            u16::try_from(s_85_8).unwrap(),
        ));
        // D s_85_10: cast reint s_85_9 -> u8
        let s_85_10: u8 = (s_85_9.value() as u8);
        // D s_85_11: cast zx s_85_10 -> bv
        let s_85_11: Bits = Bits::new(s_85_10 as u128, 4u16);
        // C s_85_12: const #0u : u8
        let s_85_12: u8 = 0;
        // C s_85_13: cast zx s_85_12 -> bv
        let s_85_13: Bits = Bits::new(s_85_12 as u128, 4u16);
        // D s_85_14: cmp-eq s_85_11 s_85_13
        let s_85_14: bool = ((s_85_11) == (s_85_13));
        // N s_85_15: assert s_85_14
        let s_85_15: () = assert!(s_85_14);
        // D s_85_16: read-var setsize:u64
        let s_85_16: u64 = fn_state.setsize;
        // D s_85_17: cast zx s_85_16 -> bv
        let s_85_17: Bits = Bits::new(s_85_16 as u128, 64u16);
        // D s_85_18: cast sx s_85_17 -> i
        let s_85_18: i128 = {
            let sign_bit = s_85_17.length() - 1;
            let mut result = s_85_17.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_85_19: cast reint s_85_18 -> i64
        let s_85_19: i64 = (s_85_18 as i64);
        // C s_85_20: const #0s : i
        let s_85_20: i128 = 0;
        // D s_85_21: cast zx s_85_19 -> i
        let s_85_21: i128 = (i128::try_from(s_85_19).unwrap());
        // D s_85_22: cmp-gt s_85_21 s_85_20
        let s_85_22: bool = ((s_85_21) > (s_85_20));
        // N s_85_23: branch s_85_22 b87 b86
        if s_85_22 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var stagesetsize:u64
        let s_86_0: u64 = fn_state.stagesetsize;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 64u16);
        // D s_86_2: cast sx s_86_1 -> i
        let s_86_2: i128 = {
            let sign_bit = s_86_1.length() - 1;
            let mut result = s_86_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_86_3: cast reint s_86_2 -> i64
        let s_86_3: i64 = (s_86_2 as i64);
        // D s_86_4: read-var setsize:u64
        let s_86_4: u64 = fn_state.setsize;
        // D s_86_5: cast zx s_86_4 -> bv
        let s_86_5: Bits = Bits::new(s_86_4 as u128, 64u16);
        // D s_86_6: cast sx s_86_5 -> i
        let s_86_6: i128 = {
            let sign_bit = s_86_5.length() - 1;
            let mut result = s_86_5.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_86_7: cast reint s_86_6 -> i64
        let s_86_7: i64 = (s_86_6 as i64);
        // D s_86_8: cast zx s_86_3 -> i
        let s_86_8: i128 = (i128::try_from(s_86_3).unwrap());
        // D s_86_9: cast zx s_86_7 -> i
        let s_86_9: i128 = (i128::try_from(s_86_7).unwrap());
        // D s_86_10: cmp-ge s_86_8 s_86_9
        let s_86_10: bool = ((s_86_8) >= (s_86_9));
        // N s_86_11: assert s_86_10
        let s_86_11: () = assert!(s_86_10);
        // N s_86_12: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var stagesetsize:u64
        let s_87_0: u64 = fn_state.stagesetsize;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 64u16);
        // D s_87_2: cast sx s_87_1 -> i
        let s_87_2: i128 = {
            let sign_bit = s_87_1.length() - 1;
            let mut result = s_87_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_87_3: cast reint s_87_2 -> i64
        let s_87_3: i64 = (s_87_2 as i64);
        // D s_87_4: read-var setsize:u64
        let s_87_4: u64 = fn_state.setsize;
        // D s_87_5: cast zx s_87_4 -> bv
        let s_87_5: Bits = Bits::new(s_87_4 as u128, 64u16);
        // D s_87_6: cast sx s_87_5 -> i
        let s_87_6: i128 = {
            let sign_bit = s_87_5.length() - 1;
            let mut result = s_87_5.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_87_7: cast reint s_87_6 -> i64
        let s_87_7: i64 = (s_87_6 as i64);
        // D s_87_8: cast zx s_87_3 -> i
        let s_87_8: i128 = (i128::try_from(s_87_3).unwrap());
        // D s_87_9: cast zx s_87_7 -> i
        let s_87_9: i128 = (i128::try_from(s_87_7).unwrap());
        // D s_87_10: cmp-le s_87_8 s_87_9
        let s_87_10: bool = ((s_87_8) <= (s_87_9));
        // N s_87_11: assert s_87_10
        let s_87_11: () = assert!(s_87_10);
        // N s_87_12: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #1u : u8
        let s_88_0: bool = true;
        // D s_88_1: write-var gs#168087 <= s_88_0
        fn_state.gs_168087 = s_88_0;
        // N s_88_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #0u : u8
        let s_89_0: u8 = 0;
        // D s_89_1: write-var nzcv <= s_89_0
        fn_state.nzcv = s_89_0;
        // D s_89_2: read-var toaddress:u64
        let s_89_2: u64 = fn_state.toaddress;
        // D s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 64u16);
        // D s_89_4: read-var setsize:u64
        let s_89_4: u64 = fn_state.setsize;
        // D s_89_5: cast zx s_89_4 -> bv
        let s_89_5: Bits = Bits::new(s_89_4 as u128, 64u16);
        // D s_89_6: add s_89_3 s_89_5
        let s_89_6: Bits = (s_89_3 + s_89_5);
        // D s_89_7: cast reint s_89_6 -> u64
        let s_89_7: u64 = (s_89_6.value() as u64);
        // D s_89_8: write-var toaddress <= s_89_7
        fn_state.toaddress = s_89_7;
        // C s_89_9: const #64s : i
        let s_89_9: i128 = 64;
        // S s_89_10: call Zeros(s_89_9)
        let s_89_10: Bits = Zeros(state, tracer, s_89_9);
        // S s_89_11: cast reint s_89_10 -> u64
        let s_89_11: u64 = (s_89_10.value() as u64);
        // S s_89_12: cast zx s_89_11 -> bv
        let s_89_12: Bits = Bits::new(s_89_11 as u128, 64u16);
        // D s_89_13: read-var setsize:u64
        let s_89_13: u64 = fn_state.setsize;
        // D s_89_14: cast zx s_89_13 -> bv
        let s_89_14: Bits = Bits::new(s_89_13 as u128, 64u16);
        // D s_89_15: sub s_89_12 s_89_14
        let s_89_15: Bits = ((s_89_12) - (s_89_14));
        // D s_89_16: cast reint s_89_15 -> u64
        let s_89_16: u64 = (s_89_15.value() as u64);
        // D s_89_17: write-var setsize <= s_89_16
        fn_state.setsize = s_89_16;
        // N s_89_18: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var accdesc:struct
        let s_90_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_90_1: call AlignmentFault(s_90_0)
        let s_90_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_90_0);
        // D s_90_2: read-var toaddress:u64
        let s_90_2: u64 = fn_state.toaddress;
        // D s_90_3: call AArch64_Abort(s_90_2, s_90_1)
        let s_90_3: () = AArch64_Abort(state, tracer, s_90_2, s_90_1);
        // N s_90_4: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var accdesc:struct
        let s_91_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_91_1: call AlignmentFault(s_91_0)
        let s_91_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_91_0);
        // D s_91_2: read-var toaddress:u64
        let s_91_2: u64 = fn_state.toaddress;
        // D s_91_3: call AArch64_Abort(s_91_2, s_91_1)
        let s_91_3: () = AArch64_Abort(state, tracer, s_91_2, s_91_1);
        // N s_91_4: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var toaddress:u64
        let s_92_0: u64 = fn_state.toaddress;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 64u16);
        // C s_92_2: const #1344u : u32
        let s_92_2: u32 = 1344;
        // D s_92_3: read-reg s_92_2:i64
        let s_92_3: i64 = {
            let value = state.read_register::<i64>(s_92_2 as isize);
            tracer.read_register(s_92_2 as isize, value);
            value
        };
        // D s_92_4: cast zx s_92_3 -> i
        let s_92_4: i128 = (i128::try_from(s_92_3).unwrap());
        // D s_92_5: call IsAligned__1(s_92_1, s_92_4)
        let s_92_5: bool = IsAligned__1(state, tracer, s_92_1, s_92_4);
        // D s_92_6: not s_92_5
        let s_92_6: bool = !s_92_5;
        // D s_92_7: write-var gs#168081 <= s_92_6
        fn_state.gs_168081 = s_92_6;
        // N s_92_8: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #9223372036854775792u : u64
        let s_93_0: u64 = 9223372036854775792;
        // D s_93_1: write-var setsize <= s_93_0
        fn_state.setsize = s_93_0;
        // N s_93_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #() : ()
        let s_94_0: () = ();
        // S s_94_1: call AArch64_IsUnprivAccessPriv(s_94_0)
        let s_94_1: bool = AArch64_IsUnprivAccessPriv(state, tracer, s_94_0);
        // D s_94_2: write-var privileged <= s_94_1
        fn_state.privileged = s_94_1;
        // N s_94_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
