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
use Mk_MPAM1_EL1_Type::*;
use MPAM1_EL1_read::*;
use u_get_MPAM1_EL1_Type_MPAMEN::*;
use MPAM1_EL1_write::*;
use u_update_MPAM1_EL1_Type_MPAMEN::*;
use common::*;
pub fn u__set_MPAM1_EL1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        gs_33800: bool,
        ga_26298: ProductType5c790c8ef59cc8b2,
        ga_26292: ProductType5c790c8ef59cc8b2,
        ga_26286: ProductType5c790c8ef59cc8b2,
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call MPAM1_EL1_read(s_0_2)
        let s_0_3: ProductType5c790c8ef59cc8b2 = MPAM1_EL1_read(state, tracer, s_0_2);
        // D s_0_4: write-var ga#26286 <= s_0_3
        fn_state.ga_26286 = s_0_3;
        // D s_0_5: read-var ga#26286.0:struct
        let s_0_5: u64 = fn_state.ga_26286._0;
        // D s_0_6: read-var r.0:struct
        let s_0_6: u64 = fn_state.r._0;
        // C s_0_7: const #0s : i
        let s_0_7: i128 = 0;
        // C s_0_8: const #54s : i
        let s_0_8: i128 = 54;
        // D s_0_9: cast zx s_0_6 -> bv
        let s_0_9: Bits = Bits::new(s_0_6 as u128, 64u16);
        // D s_0_10: bit-extract s_0_9 s_0_7 s_0_8
        let s_0_10: Bits = (Bits::new(
            ((s_0_9) >> (s_0_7)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_11: cast reint s_0_10 -> u54
        let s_0_11: u64 = (s_0_10.value() as u64);
        // C s_0_12: const #54s : i
        let s_0_12: i128 = 54;
        // C s_0_13: const #0s : i
        let s_0_13: i128 = 0;
        // D s_0_14: cast zx s_0_5 -> bv
        let s_0_14: Bits = Bits::new(s_0_5 as u128, 64u16);
        // D s_0_15: cast zx s_0_11 -> bv
        let s_0_15: Bits = Bits::new(s_0_11 as u128, 54u16);
        // C s_0_16: const #1u : u64
        let s_0_16: u64 = 1;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 64u16);
        // C s_0_18: lsl s_0_17 s_0_12
        let s_0_18: Bits = s_0_17 << s_0_12;
        // C s_0_19: sub s_0_18 s_0_17
        let s_0_19: Bits = ((s_0_18) - (s_0_17));
        // D s_0_20: and s_0_15 s_0_19
        let s_0_20: Bits = ((s_0_15) & (s_0_19));
        // D s_0_21: lsl s_0_20 s_0_13
        let s_0_21: Bits = s_0_20 << s_0_13;
        // C s_0_22: lsl s_0_19 s_0_13
        let s_0_22: Bits = s_0_19 << s_0_13;
        // C s_0_23: cmpl s_0_22
        let s_0_23: Bits = !s_0_22;
        // D s_0_24: and s_0_14 s_0_23
        let s_0_24: Bits = ((s_0_14) & (s_0_23));
        // D s_0_25: or s_0_24 s_0_21
        let s_0_25: Bits = ((s_0_24) | (s_0_21));
        // D s_0_26: cast reint s_0_25 -> u64
        let s_0_26: u64 = (s_0_25.value() as u64);
        // D s_0_27: call Mk_MPAM1_EL1_Type(s_0_26)
        let s_0_27: ProductType5c790c8ef59cc8b2 = Mk_MPAM1_EL1_Type(
            state,
            tracer,
            s_0_26,
        );
        // D s_0_28: call MPAM1_EL1_write(s_0_27)
        let s_0_28: () = MPAM1_EL1_write(state, tracer, s_0_27);
        // C s_0_29: const #() : ()
        let s_0_29: () = ();
        // S s_0_30: call MPAM1_EL1_read(s_0_29)
        let s_0_30: ProductType5c790c8ef59cc8b2 = MPAM1_EL1_read(state, tracer, s_0_29);
        // D s_0_31: write-var ga#26292 <= s_0_30
        fn_state.ga_26292 = s_0_30;
        // D s_0_32: read-var ga#26292.0:struct
        let s_0_32: u64 = fn_state.ga_26292._0;
        // D s_0_33: read-var r.0:struct
        let s_0_33: u64 = fn_state.r._0;
        // C s_0_34: const #55s : i
        let s_0_34: i128 = 55;
        // C s_0_35: const #5s : i
        let s_0_35: i128 = 5;
        // D s_0_36: cast zx s_0_33 -> bv
        let s_0_36: Bits = Bits::new(s_0_33 as u128, 64u16);
        // D s_0_37: bit-extract s_0_36 s_0_34 s_0_35
        let s_0_37: Bits = (Bits::new(
            ((s_0_36) >> (s_0_34)).value(),
            u16::try_from(s_0_35).unwrap(),
        ));
        // D s_0_38: cast reint s_0_37 -> u8
        let s_0_38: u8 = (s_0_37.value() as u8);
        // C s_0_39: const #5s : i
        let s_0_39: i128 = 5;
        // C s_0_40: const #55s : i
        let s_0_40: i128 = 55;
        // D s_0_41: cast zx s_0_32 -> bv
        let s_0_41: Bits = Bits::new(s_0_32 as u128, 64u16);
        // D s_0_42: cast zx s_0_38 -> bv
        let s_0_42: Bits = Bits::new(s_0_38 as u128, 5u16);
        // C s_0_43: const #1u : u64
        let s_0_43: u64 = 1;
        // C s_0_44: cast zx s_0_43 -> bv
        let s_0_44: Bits = Bits::new(s_0_43 as u128, 64u16);
        // C s_0_45: lsl s_0_44 s_0_39
        let s_0_45: Bits = s_0_44 << s_0_39;
        // C s_0_46: sub s_0_45 s_0_44
        let s_0_46: Bits = ((s_0_45) - (s_0_44));
        // D s_0_47: and s_0_42 s_0_46
        let s_0_47: Bits = ((s_0_42) & (s_0_46));
        // D s_0_48: lsl s_0_47 s_0_40
        let s_0_48: Bits = s_0_47 << s_0_40;
        // C s_0_49: lsl s_0_46 s_0_40
        let s_0_49: Bits = s_0_46 << s_0_40;
        // C s_0_50: cmpl s_0_49
        let s_0_50: Bits = !s_0_49;
        // D s_0_51: and s_0_41 s_0_50
        let s_0_51: Bits = ((s_0_41) & (s_0_50));
        // D s_0_52: or s_0_51 s_0_48
        let s_0_52: Bits = ((s_0_51) | (s_0_48));
        // D s_0_53: cast reint s_0_52 -> u64
        let s_0_53: u64 = (s_0_52.value() as u64);
        // D s_0_54: call Mk_MPAM1_EL1_Type(s_0_53)
        let s_0_54: ProductType5c790c8ef59cc8b2 = Mk_MPAM1_EL1_Type(
            state,
            tracer,
            s_0_53,
        );
        // D s_0_55: call MPAM1_EL1_write(s_0_54)
        let s_0_55: () = MPAM1_EL1_write(state, tracer, s_0_54);
        // C s_0_56: const #() : ()
        let s_0_56: () = ();
        // S s_0_57: call MPAM1_EL1_read(s_0_56)
        let s_0_57: ProductType5c790c8ef59cc8b2 = MPAM1_EL1_read(state, tracer, s_0_56);
        // D s_0_58: write-var ga#26298 <= s_0_57
        fn_state.ga_26298 = s_0_57;
        // D s_0_59: read-var ga#26298.0:struct
        let s_0_59: u64 = fn_state.ga_26298._0;
        // D s_0_60: read-var r.0:struct
        let s_0_60: u64 = fn_state.r._0;
        // C s_0_61: const #61s : i
        let s_0_61: i128 = 61;
        // C s_0_62: const #2s : i
        let s_0_62: i128 = 2;
        // D s_0_63: cast zx s_0_60 -> bv
        let s_0_63: Bits = Bits::new(s_0_60 as u128, 64u16);
        // D s_0_64: bit-extract s_0_63 s_0_61 s_0_62
        let s_0_64: Bits = (Bits::new(
            ((s_0_63) >> (s_0_61)).value(),
            u16::try_from(s_0_62).unwrap(),
        ));
        // D s_0_65: cast reint s_0_64 -> u8
        let s_0_65: u8 = (s_0_64.value() as u8);
        // C s_0_66: const #2s : i
        let s_0_66: i128 = 2;
        // C s_0_67: const #61s : i
        let s_0_67: i128 = 61;
        // D s_0_68: cast zx s_0_59 -> bv
        let s_0_68: Bits = Bits::new(s_0_59 as u128, 64u16);
        // D s_0_69: cast zx s_0_65 -> bv
        let s_0_69: Bits = Bits::new(s_0_65 as u128, 2u16);
        // C s_0_70: const #1u : u64
        let s_0_70: u64 = 1;
        // C s_0_71: cast zx s_0_70 -> bv
        let s_0_71: Bits = Bits::new(s_0_70 as u128, 64u16);
        // C s_0_72: lsl s_0_71 s_0_66
        let s_0_72: Bits = s_0_71 << s_0_66;
        // C s_0_73: sub s_0_72 s_0_71
        let s_0_73: Bits = ((s_0_72) - (s_0_71));
        // D s_0_74: and s_0_69 s_0_73
        let s_0_74: Bits = ((s_0_69) & (s_0_73));
        // D s_0_75: lsl s_0_74 s_0_67
        let s_0_75: Bits = s_0_74 << s_0_67;
        // C s_0_76: lsl s_0_73 s_0_67
        let s_0_76: Bits = s_0_73 << s_0_67;
        // C s_0_77: cmpl s_0_76
        let s_0_77: Bits = !s_0_76;
        // D s_0_78: and s_0_68 s_0_77
        let s_0_78: Bits = ((s_0_68) & (s_0_77));
        // D s_0_79: or s_0_78 s_0_75
        let s_0_79: Bits = ((s_0_78) | (s_0_75));
        // D s_0_80: cast reint s_0_79 -> u64
        let s_0_80: u64 = (s_0_79.value() as u64);
        // D s_0_81: call Mk_MPAM1_EL1_Type(s_0_80)
        let s_0_81: ProductType5c790c8ef59cc8b2 = Mk_MPAM1_EL1_Type(
            state,
            tracer,
            s_0_80,
        );
        // D s_0_82: call MPAM1_EL1_write(s_0_81)
        let s_0_82: () = MPAM1_EL1_write(state, tracer, s_0_81);
        // C s_0_83: const #424u : u32
        let s_0_83: u32 = 424;
        // D s_0_84: read-reg s_0_83:u8
        let s_0_84: u8 = {
            let value = state.read_register::<u8>(s_0_83 as isize);
            tracer.read_register(s_0_83 as isize, value);
            value
        };
        // C s_0_85: const #2u : u8
        let s_0_85: u8 = 2;
        // D s_0_86: cmp-lt s_0_84 s_0_85
        let s_0_86: bool = ((s_0_84) < (s_0_85));
        // D s_0_87: not s_0_86
        let s_0_87: bool = !s_0_86;
        // N s_0_88: branch s_0_87 b5 b1
        if s_0_87 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#33800 <= s_1_0
        fn_state.gs_33800 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#33800:u8
        let s_2_0: bool = fn_state.gs_33800;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
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
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call MPAM1_EL1_read(s_4_0)
        let s_4_1: ProductType5c790c8ef59cc8b2 = MPAM1_EL1_read(state, tracer, s_4_0);
        // D s_4_2: read-var r:struct
        let s_4_2: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_4_3: call _get_MPAM1_EL1_Type_MPAMEN(s_4_2)
        let s_4_3: bool = u_get_MPAM1_EL1_Type_MPAMEN(state, tracer, s_4_2);
        // D s_4_4: call _update_MPAM1_EL1_Type_MPAMEN(s_4_1, s_4_3)
        let s_4_4: ProductType5c790c8ef59cc8b2 = u_update_MPAM1_EL1_Type_MPAMEN(
            state,
            tracer,
            s_4_1,
            s_4_3,
        );
        // D s_4_5: call MPAM1_EL1_write(s_4_4)
        let s_4_5: () = MPAM1_EL1_write(state, tracer, s_4_4);
        // N s_4_6: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #432u : u32
        let s_5_0: u32 = 432;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
        // D s_5_3: cmp-lt s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) < (s_5_2));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // D s_5_5: write-var gs#33800 <= s_5_4
        fn_state.gs_33800 = s_5_4;
        // N s_5_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
